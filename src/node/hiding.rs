use crate::node::{
    CoreConstructible, DisconnectConstructible, JetConstructible, WitnessConstructible,
};
use crate::types::{Context, Error};
use crate::{Cmr, FailEntropy, HasCmr, Word};

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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hiding<'brand, N> {
    result: HidingResult<N>,
    /// Inference context for program construction.
    ///
    /// Even a "hidden" node needs an inference context
    /// because the context may be queried via [`CoreConstructible::inference_context`].
    /// When a "hidden" node is converted into an assertion via the
    /// [`CoreConstructible::case`] constructor, this context is required to build the case node.
    /// For soundness, the same context should be returned for all nodes of the same program.
    ctx: Context<'brand>,
}

type HidingResult<N> = Result<N, Cmr>;

impl<'brand, N> Hiding<'brand, N> {
    /// Create a "hidden" node with the given CMR.
    ///
    /// To enable the construction of possible parent nodes,
    /// the inference context of the current program must be passed.
    pub const fn hidden(cmr: Cmr, ctx: Context<'brand>) -> Self {
        // # Soundness
        // The hidden node introduces no type variables.
        Self {
            result: Err(cmr),
            ctx,
        }
    }

    fn hidden_cloned_ctx(&self, cmr: Cmr) -> Self {
        Self {
            result: Err(cmr),
            ctx: self.ctx.shallow_clone(),
        }
    }

    /// Access the non-hidden node inside in the wrapper.
    ///
    /// Return `None` if the wrapped node is "hidden".
    pub fn as_node(&self) -> Option<&N> {
        self.result.as_ref().ok()
    }

    /// Consume the wrapper and return the non-hidden node that was inside.
    ///
    /// Return `None` if the wrapped node is "hidden".
    pub fn get_node(self) -> Option<N> {
        self.result.ok()
    }
}

impl<N: HasCmr> Hiding<'_, N> {
    /// Ensure that the wrapped node is "hidden".
    /// Convert non-hidden nodes into "hidden" nodes with the same CMR.
    pub fn hide(self) -> Self {
        // # Soundness
        // Hiding a node means converting it into its CMR.
        // The node's type variables remain in the inference context.
        //
        // The type variables of a "hidden" child don't influence the construction of a parent,
        // because merely the child's CMR is passed to the parent constructor.
        // A CMR has no connection to any type variables.
        //
        // Hiding a node creates a CMR that is independent from the original node.
        // The CMR can be used in one part of the program
        // while the node itself is used in a different part.
        match self.result {
            Ok(node) => Self::hidden(node.cmr(), self.ctx),
            Err(..) => self,
        }
    }
}

impl<N: HasCmr> HasCmr for Hiding<'_, N> {
    fn cmr(&self) -> Cmr {
        match &self.result {
            Ok(node) => node.cmr(),
            Err(cmr) => *cmr,
        }
    }
}

// We need `N: CoreConstructible` to access the inference context.
// Because of this, implementations of `{Jet, Disconnect, Witness}Constructible`
// for `Hiding<N>` require `N: CoreConstructible`.
impl<'brand, N: CoreConstructible<'brand>> From<N> for Hiding<'brand, N> {
    fn from(node: N) -> Self {
        Self {
            ctx: node.inference_context().shallow_clone(),
            result: Ok(node),
        }
    }
}

// # Soundness
// See [`Hiding::hide`].
impl<'brand, N: CoreConstructible<'brand> + HasCmr> CoreConstructible<'brand>
    for Hiding<'brand, N>
{
    fn iden(inference_context: &Context<'brand>) -> Self {
        N::iden(inference_context).into()
    }

    fn unit(inference_context: &Context<'brand>) -> Self {
        N::unit(inference_context).into()
    }

    fn injl(child: &Self) -> Self {
        match &child.result {
            Ok(child) => N::injl(child).into(),
            Err(cmr) => child.hidden_cloned_ctx(Cmr::injl(*cmr)),
        }
    }

    fn injr(child: &Self) -> Self {
        match &child.result {
            Ok(child) => N::injr(child).into(),
            Err(cmr) => child.hidden_cloned_ctx(Cmr::injr(*cmr)),
        }
    }

    fn take(child: &Self) -> Self {
        match &child.result {
            Ok(child) => N::take(child).into(),
            Err(cmr) => child.hidden_cloned_ctx(Cmr::take(*cmr)),
        }
    }

    fn drop_(child: &Self) -> Self {
        match &child.result {
            Ok(child) => N::drop_(child).into(),
            Err(cmr) => child.hidden_cloned_ctx(Cmr::drop(*cmr)),
        }
    }

    fn comp(left: &Self, right: &Self) -> Result<Self, Error> {
        match (&left.result, &right.result) {
            (Ok(left), Ok(right)) => N::comp(left, right).map(Self::from),
            _ => Ok(left.hidden_cloned_ctx(Cmr::comp(left.cmr(), right.cmr()))),
        }
    }

    fn case(left: &Self, right: &Self) -> Result<Self, Error> {
        match (&left.result, &right.result) {
            (Ok(left), Ok(right)) => N::case(left, right).map(Self::from),
            (Err(left), Ok(right)) => N::assertr(*left, right).map(Self::from),
            (Ok(left), Err(right)) => N::assertl(left, *right).map(Self::from),
            _ => Ok(left.hidden_cloned_ctx(Cmr::case(left.cmr(), right.cmr()))),
        }
    }

    fn assertl(left: &Self, right: Cmr) -> Result<Self, Error> {
        match &left.result {
            Ok(left) => N::assertl(left, right).map(Self::from),
            _ => Ok(left.hidden_cloned_ctx(Cmr::case(left.cmr(), right))),
        }
    }

    fn assertr(left: Cmr, right: &Self) -> Result<Self, Error> {
        match &right.result {
            Ok(right) => N::assertr(left, right).map(Self::from),
            _ => Ok(right.hidden_cloned_ctx(Cmr::case(left, right.cmr()))),
        }
    }

    fn pair(left: &Self, right: &Self) -> Result<Self, Error> {
        match (&left.result, &right.result) {
            (Ok(left), Ok(right)) => N::pair(left, right).map(Self::from),
            _ => Ok(left.hidden_cloned_ctx(Cmr::pair(left.cmr(), right.cmr()))),
        }
    }

    fn fail(inference_context: &Context<'brand>, entropy: FailEntropy) -> Self {
        N::fail(inference_context, entropy).into()
    }

    fn const_word(inference_context: &Context<'brand>, word: Word) -> Self {
        N::const_word(inference_context, word).into()
    }

    fn inference_context(&self) -> &Context<'brand> {
        &self.ctx
    }
}

impl<'brand, J, N> JetConstructible<'brand, J> for Hiding<'brand, N>
where
    N: JetConstructible<'brand, J> + CoreConstructible<'brand>,
{
    fn jet(inference_context: &Context<'brand>, jet: J) -> Self {
        N::jet(inference_context, jet).into()
    }
}

impl<'brand, X, N> DisconnectConstructible<'brand, Option<X>> for Hiding<'brand, N>
where
    N: DisconnectConstructible<'brand, Option<X>> + CoreConstructible<'brand> + HasCmr,
{
    fn disconnect(left: &Self, right: &Option<X>) -> Result<Self, Error> {
        match &left.result {
            Ok(left) => N::disconnect(left, right).map(Self::from),
            Err(..) => Ok(left.hidden_cloned_ctx(Cmr::disconnect(left.cmr()))),
        }
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
