// SPDX-License-Identifier: CC0-1.0

//! General DAG iteration utilities

use std::collections::{hash_map::Entry, HashMap};
use std::fmt;
use std::sync::Arc;

use crate::node::{self, Disconnectable, Node};

/// Abstract node of a directed acyclic graph.
///
/// Tracks the arity (out-degree) of nodes in a Simplictiy program, as well
/// as whether they represent `witness` or `disconnect` combinators, which
/// are treated specially when working with the graph structure of
/// Simplicty programs.
#[derive(Clone, Debug)]
pub enum Dag<T> {
    /// Combinator with no children
    Nullary,
    /// Combinator with one child
    Unary(T),
    /// Combinator with two children
    Binary(T, T),
}

impl<T> Dag<T> {
    /// Given a DAG of one type, convert it to a DAG of another type.
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Dag<U> {
        match self {
            Dag::Nullary => Dag::Nullary,
            Dag::Unary(t) => Dag::Unary(f(t)),
            Dag::Binary(tl, tr) => Dag::Binary(f(tl), f(tr)),
        }
    }
}

/// How much sharing/expansion to do when running an iterator over a DAG
///
/// This object works by recording and looking up nodes in a DAG as they are
/// being iterated over. If the tracker says that an element has been seen
/// before, it will not be yielded again; so for example, a tracker which
/// records nodes by their IMR will implement full sharing, while a tracker
/// which claims to have never seen any node before will implement no
/// sharing at all.
pub trait SharingTracker<D: DagLike> {
    /// Marks an object as having been seen, and record the index
    /// when it was seen.
    ///
    /// If the object was already seen, does **not** update the index,
    /// and instead returns the original one.
    fn record(&mut self, object: &D, index: usize) -> Option<usize>;

    /// Check whether an object has been seen before; if so, return
    /// the index it was recorded at.
    fn seen_before(&self, object: &D) -> Option<usize>;
}

// Annoyingly we need to implement this explicitly
impl<D: DagLike> SharingTracker<D> for &mut dyn SharingTracker<D> {
    fn record(&mut self, object: &D, index: usize) -> Option<usize> {
        (**self).record(object, index)
    }
    fn seen_before(&self, object: &D) -> Option<usize> {
        (**self).seen_before(object)
    }
}

/// Do not share at all; yield every node in the expanded DAG
#[derive(Clone, Debug, Default)]
pub struct NoSharing;

impl<D: DagLike> SharingTracker<D> for NoSharing {
    fn record(&mut self, _: &D, _: usize) -> Option<usize> {
        None
    }
    fn seen_before(&self, _: &D) -> Option<usize> {
        None
    }
}

/// Share using pointer identity, i.e. yield each node only once, where two
/// nodes are the same iff they both point to the same object.
#[derive(Clone, Debug, Default)]
pub struct InternalSharing {
    map: HashMap<PointerId, usize>,
}

impl<D: DagLike> SharingTracker<D> for InternalSharing {
    fn record(&mut self, d: &D, index: usize) -> Option<usize> {
        match self.map.entry(PointerId::from(d)) {
            Entry::Occupied(occ) => Some(*occ.get()),
            Entry::Vacant(vac) => {
                vac.insert(index);
                None
            }
        }
    }
    fn seen_before(&self, d: &D) -> Option<usize> {
        self.map.get(&PointerId::from(d)).copied()
    }
}

/// Maximal sharing: share every node whose CMR and cached data match
///
/// For `RedeemNode`s, this coincides with `FullSharing`; for other
/// types of nodes it represents "as much sharing as we can currently
/// safely do".
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaxSharing<N: node::Marker> {
    map: HashMap<N::SharingId, usize>,
}

// Annoyingly we have to implement Default by hand
impl<N: node::Marker> Default for MaxSharing<N> {
    fn default() -> Self {
        MaxSharing {
            map: HashMap::default(),
        }
    }
}

impl<N: node::Marker> SharingTracker<&Node<N>> for MaxSharing<N> {
    fn record(&mut self, d: &&Node<N>, index: usize) -> Option<usize> {
        let id = d.sharing_id()?;

        match self.map.entry(id) {
            Entry::Occupied(occ) => Some(*occ.get()),
            Entry::Vacant(vac) => {
                vac.insert(index);
                None
            }
        }
    }
    fn seen_before(&self, d: &&Node<N>) -> Option<usize> {
        d.sharing_id().and_then(|id| self.map.get(&id)).copied()
    }
}

impl<N: node::Marker> SharingTracker<Arc<Node<N>>> for MaxSharing<N> {
    fn record(&mut self, d: &Arc<Node<N>>, index: usize) -> Option<usize> {
        let id = d.sharing_id()?;

        match self.map.entry(id) {
            Entry::Occupied(occ) => Some(*occ.get()),
            Entry::Vacant(vac) => {
                vac.insert(index);
                None
            }
        }
    }
    fn seen_before(&self, d: &Arc<Node<N>>) -> Option<usize> {
        d.sharing_id().and_then(|id| self.map.get(&id)).copied()
    }
}

// Need to explicitly allow swapping children for `MaxSharing`; the other
// sharing styles have blanket implementations for `D: DagLike` so they
// automatically allow it.
impl<N, D> SharingTracker<SwapChildren<D>> for MaxSharing<N>
where
    D: DagLike,
    MaxSharing<N>: SharingTracker<D>,
    N: node::Marker,
{
    fn record(&mut self, d: &SwapChildren<D>, index: usize) -> Option<usize> {
        self.record(&d.0, index)
    }

    fn seen_before(&self, d: &SwapChildren<D>) -> Option<usize> {
        self.seen_before(&d.0)
    }
}

/// Object representing pointer identity of a DAG node
///
/// Used to populate a hashset to determine whether or not a given node has
/// already been tracker by an iterator.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct PointerId(usize);

impl<D: DagLike> From<&D> for PointerId {
    fn from(dag: &D) -> Self {
        PointerId(dag.data() as *const _ as usize)
    }
}

/// Return type of the [`DagLike::rtl_post_order_iter`] method.
pub type RtlPostOrderIter<D, S> = std::iter::Map<
    PostOrderIter<SwapChildren<D>, S>,
    fn(PostOrderIterItem<SwapChildren<D>>) -> PostOrderIterItem<D>,
>;

/// A trait for any structure which has the shape of a Simplicity DAG
///
/// This should be implemented on any reference type for `Node`, though it
/// cannot be implemented on the actual structure because it assumes that
/// the node type is the same as the type contained in each variant.
pub trait DagLike: Sized {
    /// The type of the DAG node, with no references or wrappers
    type Node;

    /// A pointer to the underlying data
    fn data(&self) -> &Self::Node;

    /// Interpret the node as a DAG node
    fn as_dag_node(&self) -> Dag<Self>;

    /// Accessor for the left child of the node, if one exists
    fn left_child(&self) -> Option<Self> {
        match self.as_dag_node() {
            Dag::Nullary => None,
            Dag::Unary(sub) => Some(sub),
            Dag::Binary(left, _) => Some(left),
        }
    }

    /// Accessor for the right child of the node, if one exists
    fn right_child(&self) -> Option<Self> {
        match self.as_dag_node() {
            Dag::Nullary => None,
            Dag::Unary(_) => None,
            Dag::Binary(_, right) => Some(right),
        }
    }

    /// Number of children that the node has.
    ///
    /// This has a default implementation which simply inspects the children, but
    /// in some cases it may be more efficient for implementors to implement this
    /// manually.
    fn n_children(&self) -> usize {
        match self.as_dag_node() {
            Dag::Nullary => 0,
            Dag::Unary(..) => 1,
            Dag::Binary(..) => 2,
        }
    }

    /// Obtains an iterator of all the nodes rooted at the DAG, in right-to-left post order.
    ///
    /// An ordinary post-order iterator yields children in the order
    /// left-child, right-child, node. This one instead yields them in
    /// the order right-child, left-child, node.
    ///
    /// This is useful when implementing satisfiers, specifically for
    /// handling the `comp` combinator, by allowing the user to see the
    /// right child (and e.g. make decisions about what branches to prune
    /// or what witnesses to provide) before the left child (which may
    /// involve populating witnesses consistent with that choice).
    fn rtl_post_order_iter<S: SharingTracker<SwapChildren<Self>> + Default>(
        self,
    ) -> RtlPostOrderIter<Self, S> {
        PostOrderIter {
            index: 0,
            stack: vec![IterStackItem::unprocessed(
                SwapChildren(self),
                Previous::Root,
            )],
            tracker: Default::default(),
        }
        .map(PostOrderIterItem::unswap)
    }

    /// Obtains an iterator of all the nodes rooted at the DAG, in pre-order.
    fn pre_order_iter<S: SharingTracker<Self> + Default>(self) -> PreOrderIter<Self, S> {
        PreOrderIter {
            stack: vec![self],
            tracker: Default::default(),
        }
    }

    /// Obtains a verbose iterator of all the nodes rooted at the DAG, in pre-order.
    ///
    /// See the documentation of [`VerbosePreOrderIter`] for more information about what
    /// this does. Essentially, if you find yourself using [`Self::pre_order_iter`] and
    /// then adding a stack to manually track which items and their children have been
    /// yielded, you may be better off using this iterator instead.
    fn verbose_pre_order_iter<S: SharingTracker<Self> + Default>(
        self,
        max_depth: Option<usize>,
    ) -> VerbosePreOrderIter<Self, S>
    where
        Self: Clone,
    {
        VerbosePreOrderIter {
            stack: vec![PreOrderIterItem::initial(self, 0, None)],
            index: 0,
            max_depth,
            tracker: Default::default(),
        }
    }

    /// Obtains an iterator of all the nodes rooted at the DAG, in post order.
    ///
    /// Each node is only yielded once, at the leftmost position that it
    /// appears in the DAG.
    fn post_order_iter<S: SharingTracker<Self> + Default>(self) -> PostOrderIter<Self, S> {
        PostOrderIter {
            index: 0,
            stack: vec![IterStackItem::unprocessed(self, Previous::Root)],
            tracker: Default::default(),
        }
    }

    /// Checks whether a DAG's internal sharing (as expressed by shared pointers)
    /// matches the given target sharing.
    #[allow(clippy::wrong_self_convention)] // clippy doesn't like is_* taking a pointer by value
    fn is_shared_as<S: SharingTracker<Self> + Default>(self) -> bool
    where
        Self: Clone,
    {
        let iter_is = self.clone().post_order_iter::<InternalSharing>();
        let iter_ought = self.post_order_iter::<S>();
        for (data_is, data_ought) in iter_is.zip(iter_ought) {
            if PointerId::from(&data_is.node) != PointerId::from(&data_ought.node) {
                return false;
            }
        }
        true
    }

    /// Obtains an post-order iterator of all the nodes rooted at DAG, using the
    /// given tracker.
    ///
    /// Ordinary users will never need to use this method; but it is available to
    /// enable obscure iteration use-cases.
    fn post_order_iter_with_tracker<S: SharingTracker<Self>>(
        self,
        tracker: S,
    ) -> PostOrderIter<Self, S> {
        PostOrderIter {
            index: 0,
            stack: vec![IterStackItem::unprocessed(self, Previous::Root)],
            tracker,
        }
    }
}

/// A wrapper around a DAG-like reference that swaps the two children
///
/// This can be useful to modify the `PostOrderIter` behaviour to yield
/// right children before left children.
///
/// Since it works by relabelling the children, when iterating with this
/// adaptor, the "left" and "right" child indices will be inconsistent
/// with the returned nodes. To correct this, you need to call
/// [`PostOrderIterItem::unswap`].
///
/// To avoid confusion, this structure cannot be directly costructed.
/// Instead it is implicit in the [`DagLike::rtl_post_order_iter`]
/// method.
#[derive(Clone, Debug)]
pub struct SwapChildren<D>(D);

impl<D: DagLike> DagLike for SwapChildren<D> {
    type Node = D::Node;

    fn data(&self) -> &Self::Node {
        self.0.data()
    }

    fn as_dag_node(&self) -> Dag<Self> {
        match self.0.as_dag_node() {
            Dag::Nullary => Dag::Nullary,
            Dag::Unary(sub) => Dag::Unary(SwapChildren(sub)),
            Dag::Binary(left, right) => Dag::Binary(SwapChildren(right), SwapChildren(left)),
        }
    }
}

impl<'a, N: node::Marker> DagLike for &'a Node<N> {
    type Node = Node<N>;

    fn data(&self) -> &Node<N> {
        self
    }

    #[rustfmt::skip]
    fn as_dag_node(&self) -> Dag<Self> {
        match self.inner() {
            node::Inner::Iden
            | node::Inner::Unit
            | node::Inner::Fail(..)
            | node::Inner::Jet(..)
            | node::Inner::Word(..) => Dag::Nullary,
            node::Inner::InjL(ref sub)
            | node::Inner::InjR(ref sub)
            | node::Inner::Take(ref sub)
            | node::Inner::Drop(ref sub)
            | node::Inner::AssertL(ref sub, _)
            | node::Inner::AssertR(_, ref sub) => Dag::Unary(sub),
            node::Inner::Comp(ref left, ref right)
            | node::Inner::Case(ref left, ref right)
            | node::Inner::Pair(ref left, ref right) => Dag::Binary(left, right),
            node::Inner::Disconnect(ref left, ref right) => right.disconnect_dag_ref(left),
            node::Inner::Witness(..) => Dag::Nullary,
        }
    }
}

impl<N: node::Marker> DagLike for Arc<Node<N>> {
    type Node = Node<N>;

    fn data(&self) -> &Node<N> {
        self
    }

    #[rustfmt::skip]
    fn as_dag_node(&self) -> Dag<Self> {
        match self.inner() {
            node::Inner::Iden
            | node::Inner::Unit
            | node::Inner::Fail(..)
            | node::Inner::Jet(..)
            | node::Inner::Word(..) => Dag::Nullary,
            node::Inner::InjL(ref sub)
            | node::Inner::InjR(ref sub)
            | node::Inner::Take(ref sub)
            | node::Inner::Drop(ref sub)
            | node::Inner::AssertL(ref sub, _)
            | node::Inner::AssertR(_, ref sub) => Dag::Unary(Arc::clone(sub)),
            node::Inner::Comp(ref left, ref right)
            | node::Inner::Case(ref left, ref right)
            | node::Inner::Pair(ref left, ref right) => Dag::Binary(Arc::clone(left), Arc::clone(right)),
            node::Inner::Disconnect(ref left, ref right) => right.clone().disconnect_dag_arc(Arc::clone(left)),
            node::Inner::Witness(..) => Dag::Nullary,
        }
    }
}

enum Child<D: DagLike> {
    None,
    Repeat { idx: usize },
    New(D),
}

#[derive(Clone, Debug)]
enum Previous {
    /// This is the root element and there are no previous elements
    Root,
    /// This is a left child and the previous element is its parent
    ParentLeft,
    /// This is a left child and the previous element is its sibling
    SiblingLeft,
    /// This is a right child and the previous element is its parent
    ParentRight,
}

#[derive(Clone, Debug)]
struct IterStackItem<D> {
    /// The element on the stack
    elem: D,
    /// Whether we have dealt with this item (and pushed its children,
    /// if any) yet.
    processed: bool,
    /// If the item has been processed, the index of its left child, if known
    left_idx: Option<usize>,
    /// If the item has been processed, the index of its right child, if known
    right_idx: Option<usize>,
    /// Whether the element is a left- or right-child of its parent
    previous: Previous,
}

impl<D: DagLike> IterStackItem<D> {
    /// Constructor for a new stack item with a given element and relationship
    /// to its parent.
    fn unprocessed(elem: D, previous: Previous) -> Self {
        IterStackItem {
            elem,
            processed: false,
            left_idx: None,
            right_idx: None,
            previous,
        }
    }

    fn left_child<V: SharingTracker<D>>(&self, tracker: &V) -> Child<D> {
        match self.elem.left_child() {
            Some(child) => match tracker.seen_before(&child) {
                Some(idx) => Child::Repeat { idx },
                None => Child::New(child),
            },
            None => Child::None,
        }
    }

    fn right_child<V: SharingTracker<D>>(&self, tracker: &V) -> Child<D> {
        match self.elem.right_child() {
            Some(child) => match tracker.seen_before(&child) {
                Some(idx) => Child::Repeat { idx },
                None => Child::New(child),
            },
            None => Child::None,
        }
    }
}

/// Iterates over a DAG in _post order_.
///
/// That means nodes are yielded in the order (left child, right child, parent).
/// Nodes may be repeated or not, based on the `S` parameter which defines how
/// the iterator treats sharing.
#[derive(Clone, Debug)]
pub struct PostOrderIter<D, S> {
    /// The index of the next item to be yielded
    index: usize,
    /// A stack of elements to be yielded; each element is a node, then its left
    /// and right children (if they exist and if they have been yielded already)
    stack: Vec<IterStackItem<D>>,
    /// Data which tracks which nodes have been yielded already and therefore
    /// should be skipped.
    tracker: S,
}

/// A set of data yielded by a `PostOrderIter`
#[derive(Clone, Debug)]
pub struct PostOrderIterItem<D> {
    /// The actual node data
    pub node: D,
    /// The index of this node (equivalent to if you'd called `.enumerate()` on
    /// the iterator)
    pub index: usize,
    /// The index of this node's left child, if it has a left child
    pub left_index: Option<usize>,
    /// The index of this node's right child, if it has a left child
    pub right_index: Option<usize>,
}

impl<D: DagLike> PostOrderIterItem<SwapChildren<D>> {
    /// When iterating in right-to-left mode using the [`SwapChildren`] adaptor,
    /// use this method to correct the child indices. See documentation on
    /// [`SwapChildren`] or [`DagLike::rtl_post_order_iter`].
    fn unswap(mut self) -> PostOrderIterItem<D> {
        if matches!(self.node.as_dag_node(), Dag::Binary(..)) {
            std::mem::swap(&mut self.left_index, &mut self.right_index);
        }
        PostOrderIterItem {
            node: self.node.0,
            index: self.index,
            left_index: self.left_index,
            right_index: self.right_index,
        }
    }
}

impl<D: DagLike, S: SharingTracker<D>> Iterator for PostOrderIter<D, S> {
    type Item = PostOrderIterItem<D>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Look at the current top item on the stack
            if let Some(mut current) = self.stack.pop() {
                if !current.processed {
                    current.processed = true;
                    // When we first encounter an item, it is completely unknown; it is
                    // nominally the next item to be yielded, but it might have children,
                    // and if so, they come first
                    match (
                        current.left_child(&self.tracker),
                        current.right_child(&self.tracker),
                    ) {
                        // No children is easy, just mark it processed and iterate.
                        // (We match _ for the right child but we know that if the left one
                        // is Child::None, then the right one will be Child::None as well.)
                        (Child::None, _) => {
                            self.stack.push(current);
                        }
                        // Only a left child, already yielded
                        (Child::Repeat { idx }, Child::None) => {
                            current.left_idx = Some(idx);
                            self.stack.push(current);
                        }
                        // Only a left child, new
                        (Child::New(child), Child::None) => {
                            self.stack.push(current);
                            self.stack
                                .push(IterStackItem::unprocessed(child, Previous::ParentLeft));
                        }
                        // Two children, both already yielded
                        (Child::Repeat { idx: lidx }, Child::Repeat { idx: ridx }) => {
                            current.left_idx = Some(lidx);
                            current.right_idx = Some(ridx);
                            self.stack.push(current);
                        }
                        // Two children, one yielded already
                        (Child::New(child), Child::Repeat { idx }) => {
                            current.right_idx = Some(idx);
                            self.stack.push(current);
                            self.stack
                                .push(IterStackItem::unprocessed(child, Previous::ParentLeft));
                        }
                        (Child::Repeat { idx }, Child::New(child)) => {
                            current.left_idx = Some(idx);
                            self.stack.push(current);
                            self.stack
                                .push(IterStackItem::unprocessed(child, Previous::ParentRight));
                        }
                        // Two children, neither yielded already
                        (Child::New(lchild), Child::New(rchild)) => {
                            self.stack.push(current);
                            self.stack
                                .push(IterStackItem::unprocessed(rchild, Previous::ParentRight));
                            self.stack
                                .push(IterStackItem::unprocessed(lchild, Previous::SiblingLeft));
                        }
                    }
                } else {
                    // The second time we encounter an item, we have dealt with its children,
                    // updated the child indices for this item, and are now ready to yield it
                    // rather than putting it back in the stack. However, while dealing with
                    // its children, we may have already yielded it (which can happen because
                    // this is a DAG, not a tree). In this case we want to skip it.
                    //
                    // Check whether this is the case, and record the item's new index if not.
                    let current_index = self.tracker.record(&current.elem, self.index);
                    let already_yielded = current_index.is_some();
                    let current_index = current_index.unwrap_or(self.index);

                    // Then, update the item's parents and/or sibling, to make sure that its
                    // parent has the correct child index. We need to do this whether or not
                    // we're going to skip the element.
                    let stack_len = self.stack.len();
                    match current.previous {
                        Previous::Root => {
                            assert_eq!(stack_len, 0);
                        }
                        Previous::ParentLeft => {
                            assert!(self.stack[stack_len - 1].processed);
                            self.stack[stack_len - 1].left_idx = Some(current_index);
                        }
                        Previous::ParentRight => {
                            assert!(self.stack[stack_len - 1].processed);
                            self.stack[stack_len - 1].right_idx = Some(current_index);
                        }
                        Previous::SiblingLeft => {
                            assert!(self.stack[stack_len - 2].processed);
                            self.stack[stack_len - 2].left_idx = Some(current_index);
                        }
                    }
                    // Having updated the parent indices, so that our iterator is in a
                    // consistent state, we can safely skip here if the current item is
                    // already yielded.
                    if already_yielded {
                        continue;
                    }

                    self.index += 1;
                    return Some(PostOrderIterItem {
                        node: current.elem,
                        index: current_index,
                        left_index: current.left_idx,
                        right_index: current.right_idx,
                    });
                }
            } else {
                // If there is nothing on the stack we are done.
                return None;
            }
        }
    }
}

impl<'a, N: node::Marker, S: SharingTracker<&'a Node<N>> + Clone> PostOrderIter<&'a Node<N>, S> {
    /// Adapt the iterator to only yield witnesses
    ///
    /// The witnesses are yielded in the order in which they appear in the DAG
    /// *except* that each witness is only yielded once, and future occurences
    /// are skipped.
    pub fn into_witnesses(self) -> impl Iterator<Item = &'a N::Witness> + Clone {
        self.filter_map(|data| {
            if let node::Inner::Witness(value) = data.node.inner() {
                Some(value)
            } else {
                None
            }
        })
    }
}

impl<D: DagLike, S: SharingTracker<D>> PostOrderIter<D, S> {
    /// Display the DAG as an indexed list in post order.
    ///
    /// `display_body()` formats the node body in front of the node indices.
    /// `display_aux()` formats auxiliary items after the node indices.
    pub fn into_display<F, G>(
        self,
        f: &mut fmt::Formatter<'_>,
        mut display_body: F,
        mut display_aux: G,
    ) -> fmt::Result
    where
        F: FnMut(&D, &mut fmt::Formatter<'_>) -> fmt::Result,
        G: FnMut(&D, &mut fmt::Formatter<'_>) -> fmt::Result,
    {
        for data in self {
            write!(f, "{}: ", data.index)?;
            display_body(&data.node, f)?;

            if let Some(i_abs) = data.left_index {
                if let Some(j_abs) = data.right_index {
                    write!(f, "({}, {})", i_abs, j_abs)?;
                } else {
                    write!(f, "({})", i_abs)?;
                }
            }
            display_aux(&data.node, f)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

/// Iterates over a DAG in _pre order_.
///
/// Unlike the post-order iterator, this one does not keep track of indices
/// (this would be impractical since when we yield a node we have not yet
/// yielded its children, so we cannot know their indices). If you do need
/// the indices for some reason, the best strategy may be to run the
/// post-order iterator, collect into a vector, then iterate through that
/// backward.
#[derive(Clone, Debug)]
pub struct PreOrderIter<D, S> {
    /// A stack of elements to be yielded. As items are yielded, their right
    /// children are put onto the stack followed by their left, so that the
    /// appropriate one will be yielded on the next iteration.
    stack: Vec<D>,
    /// Data which tracks which nodes have been yielded already and therefore
    /// should be skipped.
    tracker: S,
}

impl<D: DagLike, S: SharingTracker<D>> Iterator for PreOrderIter<D, S> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        // This algorithm is _significantly_ simpler than the post-order one,
        // mainly because we don't care about child indices.
        while let Some(top) = self.stack.pop() {
            // Only yield if this is the first time we have seen this node.
            if self.tracker.record(&top, 0).is_none() {
                // If so, mark its children as to-yield, then yield it.
                if let Some(child) = top.right_child() {
                    self.stack.push(child);
                }
                if let Some(child) = top.left_child() {
                    self.stack.push(child);
                }
                return Some(top);
            }
        }
        None
    }
}

/// Iterates over a DAG in "verbose pre order", yielding extra state changes.
///
/// This yields nodes followed by their children, followed by the node *again*
/// after each child. This means that each node will be yielded a total of
/// (n+1) times, where n is its number of children.
///
/// The different times that a node is yielded can be distinguished by looking
/// at the [`PreOrderIterItem::n_children_yielded`]  (which, in particular,
/// will be 0 on the first yield) and [`PreOrderIterItem::is_complete`] (which
/// will be true on the last yield) fields of the yielded item.
///
/// If you use this iterator with a non-trivial sharing tracker, then any
/// items which have been initially yielded before will not be initially
/// yielded again.
///
/// If node's *children* have been initially yielded before, they will be
/// skipped, but the node itself will still be re-yielded as many times
/// as it has children.
#[derive(Clone, Debug)]
pub struct VerbosePreOrderIter<D, S> {
    /// A stack of elements to be yielded. As items are yielded, their right
    /// children are put onto the stack followed by their left, so that the
    /// appropriate one will be yielded on the next iteration.
    stack: Vec<PreOrderIterItem<D>>,
    /// Maximum depth (distance from root) that the iterator will go to. Any
    /// children at a greater depth will not be yielded. However, the parent
    /// of such children will still be yielded multiple times, one for each
    /// (unyielded) child, with `n_children_yielded` incremented as though
    /// the children had been yielded.
    ///
    /// To determine whether pruning has happened, you should manually check
    /// the [`PreOrderIterItem::depth`] field against the maximum depth that
    /// you set when constructing the iterator.
    max_depth: Option<usize>,
    /// The index of the next item to be yielded.
    ///
    /// Note that unlike the [`PostOrderIter`], this value is not monotonic
    /// and not equivalent to just using `enumerate` on the iterator, because
    /// elements may be yielded multiple times.
    index: usize,
    /// Data which tracks which nodes have been yielded already and therefore
    /// should be skipped.
    tracker: S,
}

impl<D: DagLike + Clone, S: SharingTracker<D>> Iterator for VerbosePreOrderIter<D, S> {
    type Item = PreOrderIterItem<D>;

    fn next(&mut self) -> Option<Self::Item> {
        // This algorithm is still simpler than the post-order one, because while
        // we care about node indices, we don't care about their childrens' indices.
        while let Some(mut top) = self.stack.pop() {
            // If this is the *first* time we'd be yielding this element, act
            // like the non-verbose pre-order iterator.
            if top.n_children_yielded == 0 {
                // If we've seen the node before, skip it.
                if self.tracker.record(&top.node, 0).is_some() {
                    continue;
                }
                // Set its index.
                top.index = self.index;
                self.index += 1;
            }
            // Push the next child.
            match (top.n_children_yielded, top.node.n_children()) {
                (0, 0) => {}
                (0, n) => {
                    self.stack.push(top.clone().increment(n == 1));
                    if top.depth < self.max_depth.unwrap_or(top.depth + 1) {
                        let child = top.node.left_child().unwrap();
                        self.stack.push(PreOrderIterItem::initial(
                            child,
                            top.depth + 1,
                            Some(top.node.clone()),
                        ));
                    }
                }
                (1, 0) => unreachable!(),
                (1, 1) => {}
                (1, _) => {
                    self.stack.push(top.clone().increment(true));
                    if top.depth < self.max_depth.unwrap_or(top.depth + 1) {
                        let child = top.node.right_child().unwrap();
                        self.stack.push(PreOrderIterItem::initial(
                            child,
                            top.depth + 1,
                            Some(top.node.clone()),
                        ));
                    }
                }
                (x, y) => {
                    debug_assert_eq!((x, y), (2, 2));
                }
            }
            // Then yield the element.
            return Some(top);
        }
        None
    }
}

/// A set of data yielded by a [`VerbosePreOrderIter`].
#[derive(Clone, Debug)]
pub struct PreOrderIterItem<D> {
    /// The actual element being yielded.
    pub node: D,
    /// The parent of this node. `None` for the initial node, but will be
    /// populated for all other nodes.
    pub parent: Option<D>,
    /// The index when the element was first yielded.
    pub index: usize,
    /// The distance of this element from the initial node. 0 for the initial
    /// node itself.
    pub depth: usize,
    /// How many of this item's children have been yielded.
    ///
    /// This can also be interpreted as a count of how many times this
    /// item has been yielded before.
    pub n_children_yielded: usize,
    /// Whether this item is done (will not be yielded again).
    pub is_complete: bool,
}

impl<D: DagLike + Clone> PreOrderIterItem<D> {
    /// Creates a `PreOrderIterItem` which yields a given element for the first time.
    ///
    /// Marks the index as 0. The index must be manually set before yielding.
    fn initial(d: D, depth: usize, parent: Option<D>) -> Self {
        PreOrderIterItem {
            is_complete: matches!(d.as_dag_node(), Dag::Nullary),
            node: d,
            parent,
            index: 0,
            depth,
            n_children_yielded: 0,
        }
    }

    /// Creates a `PreOrderIterItem` which yields a given element again.
    fn increment(self, is_complete: bool) -> Self {
        PreOrderIterItem {
            node: self.node,
            index: self.index,
            depth: self.depth,
            parent: self.parent,
            n_children_yielded: self.n_children_yielded + 1,
            is_complete,
        }
    }
}
