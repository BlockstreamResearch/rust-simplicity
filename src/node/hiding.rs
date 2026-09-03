use crate::jet::Jet;
use crate::node::{CoreConstructible, DisconnectConstructible, WitnessConstructible};
use crate::types::{Arrow, Context, Error};
use crate::{Cmr, FailEntropy, HasCmr, Word};

#[derive(Clone, Debug)]
enum HidingInner<'brand, N> {
    Node(N),
    Hidden { cmr: Cmr, arrow: Arrow<'brand> },
}

/// Wrapper that allows a node to be "hidden" during program construction.
///
/// ## Program construction
///
/// When a program is constructed in post-order,
/// the parent node is created based on its children.
/// We use this fact to introduce special branching logic:
///
/// 1. A `case` node with a left "hidden" child and a right non-hidden child becomes `assertr`.
/// 2. A `case` node with a left non-hidden child and a right "hidden" child becomes `assertl`.
/// 3. Otherwise, any node with "hidden" children becomes itself "hidden" with an updated CMR.
/// 4. Any node with non-hidden children remains unchanged.
///
/// The program can be extracted from the wrapper when construction is finished.
/// The program is invalid if the root node is "hidden".
///
/// ## Wrapping
///
/// A node can be wrapped via [`Hiding::from`] to add hiding support.
/// A wrapped node can be converted into a "hidden" node via [`Hiding::hide`].
/// Finally, a "hidden" node can be manually created via [`Hiding::hidden`].
///
/// ## Virtual hidden nodes
///
/// The wrapper merely _simulates_ hidden nodes.
/// At no point are actual hidden nodes created.
/// To stress this fact, I write "hidden" in quotation marks.
#[derive(Clone, Debug)]
pub struct Hiding<'brand, N> {
    inner: HidingInner<'brand, N>,
}

impl<'brand, N> Hiding<'brand, N> {
    /// Create a "hidden" node with the given CMR.
    ///
    /// To enable the construction of possible parent nodes,
    /// the inference context of the current program must be passed.
    pub fn hidden(cmr: Cmr, ctx: &Context<'brand>) -> Self {
        Self {
            inner: HidingInner::Hidden {
                cmr,
                arrow: Arrow::hidden(ctx),
            },
        }
    }

    /// If the node is not hidden, apply a function to the underlying node. If it is hidden,
    /// apply a function to its CMR.
    #[inline]
    pub fn map_ref<M>(
        &self,
        mapfn: impl FnOnce(&N) -> M,
        cmrfn: impl FnOnce(Cmr) -> Cmr,
    ) -> Hiding<'brand, M> {
        use core::convert::Infallible;
        match self.map_ref_result::<_, Infallible>(|node| Ok(mapfn(node)), cmrfn) {
            Ok(res) => res,
            Err(inf) => match inf {},
        }
    }

    /// If the node is not hidden, apply a function to the underlying node. If it is hidden,
    /// do nothing.
    #[inline]
    pub fn map_ref_result<M, Err>(
        &self,
        mapfn: impl FnOnce(&N) -> Result<M, Err>,
        cmrfn: impl FnOnce(Cmr) -> Cmr,
    ) -> Result<Hiding<'brand, M>, Err> {
        Ok(Hiding {
            inner: match self.inner {
                HidingInner::Node(ref n) => HidingInner::Node(mapfn(n)?),
                HidingInner::Hidden { cmr, ref arrow } => HidingInner::Hidden {
                    cmr: cmrfn(cmr),
                    arrow: arrow.shallow_clone(),
                },
            },
        })
    }

    /// Access the non-hidden node inside in the wrapper.
    ///
    /// Return `None` if the wrapped node is "hidden".
    pub fn as_node(&self) -> Option<&N> {
        match self.inner {
            HidingInner::Node(ref n) => Some(n),
            HidingInner::Hidden { .. } => None,
        }
    }

    /// Consume the wrapper and return the non-hidden node that was inside.
    ///
    /// Return `None` if the wrapped node is "hidden".
    pub fn into_node(self) -> Option<N> {
        match self.inner {
            HidingInner::Node(n) => Some(n),
            HidingInner::Hidden { .. } => None,
        }
    }
}

impl<'brand, N: HasCmr + CoreConstructible<'brand>> Hiding<'brand, N> {
    /// If neither node is hidden, apply a function to the underlying nodes to produce a new
    /// non-hidden node. If either node is hidden, apply an alternate function to the CMRs
    /// to produce a new CMR.
    ///
    /// Non-public since the API is kinda messy.
    fn zip_ref<M: CoreConstructible<'brand>>(
        &self,
        other: &Self,
        node_zipfn: impl FnOnce(&N, &N) -> Result<M, Error>,
        cmr_zipfn: impl FnOnce(Cmr, Cmr) -> Cmr,
    ) -> Result<Hiding<'brand, M>, Error> {
        Ok(Hiding {
            inner: match (&self.inner, &other.inner) {
                (HidingInner::Node(ref left), HidingInner::Node(ref right)) => {
                    node_zipfn(left, right).map(HidingInner::Node)?
                }
                _ => {
                    self.inference_context()
                        .check_eq(other.inference_context())?;
                    HidingInner::Hidden {
                        cmr: cmr_zipfn(self.cmr(), other.cmr()),
                        arrow: Arrow::hidden(self.inference_context()),
                    }
                }
            },
        })
    }

    /// Replace the node, if any, with its CMR; replace its type arrow with a new free arrow.
    ///
    /// Once hidden, a node's original type arrow loses its original bounds. In effect, a
    /// hidden node is a completely separate node from its "original" node and is typechecked
    /// and shared independently.
    pub fn hide(self) -> Self {
        match self.inner {
            HidingInner::Node(node) => Self {
                inner: HidingInner::Hidden {
                    cmr: node.cmr(),
                    arrow: Arrow::hidden(node.inference_context()),
                },
            },
            HidingInner::Hidden { .. } => self,
        }
    }
}

impl<N: HasCmr> HasCmr for Hiding<'_, N> {
    fn cmr(&self) -> Cmr {
        match self.inner {
            HidingInner::Node(ref node) => node.cmr(),
            HidingInner::Hidden { cmr, .. } => cmr,
        }
    }
}

impl<'brand, N> From<N> for Hiding<'brand, N> {
    fn from(node: N) -> Self {
        Self {
            inner: HidingInner::Node(node),
        }
    }
}

// # Soundness
// See [`Hiding::hide`].
impl<'brand, N: HasCmr + CoreConstructible<'brand>> CoreConstructible<'brand>
    for Hiding<'brand, N>
{
    fn iden(inference_context: &Context<'brand>) -> Self {
        N::iden(inference_context).into()
    }

    fn unit(inference_context: &Context<'brand>) -> Self {
        N::unit(inference_context).into()
    }

    fn injl(child: &Self) -> Self {
        child.map_ref(N::injl, Cmr::injl)
    }

    fn injr(child: &Self) -> Self {
        child.map_ref(N::injr, Cmr::injr)
    }

    fn take(child: &Self) -> Self {
        child.map_ref(N::take, Cmr::take)
    }

    fn drop_(child: &Self) -> Self {
        child.map_ref(N::drop_, Cmr::drop)
    }

    fn comp(left: &Self, right: &Self) -> Result<Self, Error> {
        left.zip_ref(right, N::comp, Cmr::comp)
    }

    fn case(left: &Self, right: &Self) -> Result<Self, Error> {
        use HidingInner as I;

        left.inference_context()
            .check_eq(right.inference_context())?;
        let inner = match (&left.inner, &right.inner) {
            (I::Node(left), I::Node(right)) => I::Node(N::case(left, right)?),
            (I::Hidden { cmr, .. }, I::Node(right)) => I::Node(N::assertr(*cmr, right)?),
            (I::Node(left), I::Hidden { cmr, .. }) => I::Node(N::assertl(left, *cmr)?),
            (I::Hidden { cmr: l_cmr, .. }, I::Hidden { cmr: r_cmr, .. }) => I::Hidden {
                cmr: Cmr::case(*l_cmr, *r_cmr),
                arrow: Arrow::hidden(left.inference_context()),
            },
        };

        Ok(Self { inner })
    }

    fn assertl(left: &Self, right: Cmr) -> Result<Self, Error> {
        left.map_ref_result(
            |left| N::assertl(left, right),
            |lcmr| Cmr::case(lcmr, right),
        )
    }

    fn assertr(left: Cmr, right: &Self) -> Result<Self, Error> {
        right.map_ref_result(
            |right| N::assertr(left, right),
            |rcmr| Cmr::case(left, rcmr),
        )
    }

    fn pair(left: &Self, right: &Self) -> Result<Self, Error> {
        left.zip_ref(right, N::pair, Cmr::pair)
    }

    fn fail(inference_context: &Context<'brand>, entropy: FailEntropy) -> Self {
        N::fail(inference_context, entropy).into()
    }

    fn const_word(inference_context: &Context<'brand>, word: Word) -> Self {
        N::const_word(inference_context, word).into()
    }

    fn jet(inference_context: &Context<'brand>, jet: &dyn Jet) -> Self {
        N::jet(inference_context, jet).into()
    }

    fn arrow(&self) -> &Arrow<'brand> {
        match self.inner {
            HidingInner::Node(ref node) => node.arrow(),
            HidingInner::Hidden { ref arrow, .. } => arrow,
        }
    }
}

impl<'brand, X, N> DisconnectConstructible<'brand, Option<X>> for Hiding<'brand, N>
where
    N: DisconnectConstructible<'brand, Option<X>> + CoreConstructible<'brand> + HasCmr,
{
    fn disconnect(left: &Self, right: &Option<X>) -> Result<Self, Error> {
        left.map_ref_result(|left| N::disconnect(left, right), Cmr::disconnect)
    }
}

impl<'brand, W, N> WitnessConstructible<'brand, W> for Hiding<'brand, N>
where
    N: WitnessConstructible<'brand, W> + CoreConstructible<'brand>,
{
    fn witness(inference_context: &Context<'brand>, witness: W) -> Self {
        N::witness(inference_context, witness).into()
    }
}
