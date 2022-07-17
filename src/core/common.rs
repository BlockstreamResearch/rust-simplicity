// TODO: add generic iterator once available in Rust
/// Linear program that consists of nodes that form a DAG _(directed acyclic graph)_.
pub trait LinearProgram: IntoIterator {
    /// Node in the program
    type Node;

    /// Return whether the program is empty.
    fn is_empty(&self) -> bool;

    /// Return the length of the program.
    fn len(&self) -> usize;

    /// Return the root node of the program.
    ///
    /// Panics if the program is empty.
    fn root(&self) -> &Self::Node;
}
