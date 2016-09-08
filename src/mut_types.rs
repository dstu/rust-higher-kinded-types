//! Types that can traverse the graph and may modify its structure. These types
//! are backed by a mutable borrow of graph data, so multiple active
//! (unborrowed) instances cannot exist.

use super::{GraphTypes, BoundedIterator};
use super::nav_types::NavTypes;

pub trait MutTypes<'a>: NavTypes<'a> {
    type MutVertex: Vertex<'a, Types=Self>;
    type MutEdge: Edge<'a, Types=Self>;
    type MutInEdgeCollection: InEdgeCollection<'a, Types=Self>;
    type MutOutEdgeCollection: OutEdgeCollection<'a, Types=Self>;
}

pub trait Vertex<'a>: Sized {
    type Types: MutTypes<'a, MutVertex=Self>;

    fn data(&self) -> &<<Self as Vertex<'a>>::Types as GraphTypes>::VertexData;

    /// Since we have mutable access to the underlying graph, we can get a
    /// mutable borrow of its actual contents.
    fn data_mut(&mut self) -> &mut <<Self as Vertex<'a>>::Types as GraphTypes>::VertexData;

    /// Consumes `self` entirely and passes the underlying borrow to a list of
    /// outgoing edges.
    fn to_out_edges(self) -> <<Self as Vertex<'a>>::Types as MutTypes<'a>>::MutOutEdgeCollection;

    /// Consumes `self` entirely and passes the underlying borrow to a list of
    /// incomign edges.
    fn to_in_edges(self) -> <<Self as Vertex<'a>>::Types as MutTypes<'a>>::MutInEdgeCollection;

    /// Vertex and its kindred in the mut_types module are backed by a mutable
    /// borrow of a graph structure. This means that it is not possible to have
    /// multiple live instances of impls of these traits (say, a `Vertex` and an
    /// `Edge`) that are backed by the same graph. For flexibility, it is
    /// desirable to be able to create a read-only borrow from a
    /// `mut_types::Vertex`, so that we can do something like:
    ///
    /// ```rust,ignore
    /// let mut vertex = ...;  // Get a mut_types::Vertex<'a> impl.
    /// add_children(&mut vertex);  // Modify graph topology.
    /// {
    ///   let mut all_children = Vec::new();
    ///   // Read-only borrow of vertex. Value returned implements nav_types::OutEdgeCollection.
    ///   let children_collection = vertex.out_edges();
    ///   for edge in children_collection.iter() {
    ///     all_children.push(edge);  // Each of these edges shares the borrow created by
    ///                               // children_collection.
    ///   }
    ///   do_something(&all_children);  // etc.
    /// }  // Borrow of vertex ends. Now we can modify graph topology through vertex again.
    /// ```
    ///
    /// When we reborrow by calling `vertex.out_edges()` in the above snippet,
    /// we create an instance of a type that implements
    /// `nav_types::OutEdgeCollection<'s>`, for the lifetime `'s` that is
    /// created for the borrow of vertex when it is passed as the `&self`
    /// parameter of out_edges. Unfortunately for us the return type of
    /// `out_edges()` needs to include this lifetime, and it is not known
    /// statically. A different lifetime must be created for each distinct
    /// borrow of a `mut_types::Vertex` impl. To be able to do this, we need to
    /// be able to apply the type `nav_types::OutEdgeCollection` to the lifetime
    /// `'s`. This requires polymorphic types that are functions of lifetimes.
    ///
    /// For simple relationships between a lifetime that is already bound (like
    /// the `'a` of `Vertex<'a>`) and on that is created in a new scope (like
    /// the `'s` of `fn out_edges<'s>(&'s self)`), we know that `'a` outlives
    /// `'s`, so it looks a lot like all the information needed to generate the
    /// appropriate type (like `Vertex<'s>`) is available. Unfortunately, the
    /// compiler balks with the error:
    ///
    /// ```txt
    /// error[E0308]: mismatched types
    /// fn out_edges<'s>(&'s self) -> <<Self as Vertex<'s>>::Types as NavTypes<'s>>::NavOutEdgeCollection where 'a: 's;
    ///
    /// lifetime mismatch
    ///
    /// note: expected type `mut_types::MutTypes<'a>`
    /// note:    found type `mut_types::MutTypes<'s>`
    /// note: the lifetime 's as defined on unknown free region bounded by scope CodeExtent(85/DestructionScope(309))...
    /// note: ...does not necessarily outlive the lifetime 'a as defined on unknown free region bounded by scope CodeExtent(85/DestructionScope(309))
    /// ```
    ///
    /// This is the case even when we state that 'a outlives 's, as in:
    ///
    /// ```rust,ignore
    /// fn out_edges<'s>(&'s self) -> <<Self as Vertex<'s>>::Types as NavTypes<'s>>::NavOutEdgeCollection where 'a: 's;
    /// fn in_edges<'s>(&'s self) -> <<Self as Vertex<'s>>::Types as NavTypes<'s>>::NavInEdgeCollection where 'a: 's;
    /// fn out_edges_mut<'s>(&'s mut self) -> <<Self as Vertex<'s>>::Types as MutTypes<'s>>::MutOutEdgeCollection where 'a: 's;
    /// fn in_edges_mut<'s>(&'s mut self) -> <<Self as Vertex<'s>>::Types as MutTypes<'s>>::MutInEdgeCollection where 'a: 's;
    /// ```
    fn out_edges(&self);
}

pub trait Edge<'a>: Sized {
    type Types: MutTypes<'a, MutEdge=Self>;
}

pub trait InEdgeCollection<'a>: Sized {
    type Types: MutTypes<'a, MutInEdgeCollection=Self>;
    type Iter: BoundedIterator<'a>;
}

pub trait OutEdgeCollection<'a>: Sized {
    type Types: MutTypes<'a, MutOutEdgeCollection=Self>;
    type Iter: BoundedIterator<'a>;
}
