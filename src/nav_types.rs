//! Types that can traverse the graph and cannot modify its structure. These
//! types are backed by a read-only borrow of graph data, so multiple instances
//! of them may coexist.
//!
//! These traits may be thought of as fat pointers into some graph structure,
//! augmented operations to navigate it.

use super::{GraphTypes, BoundedIterator};

pub trait NavTypes<'a>: GraphTypes {
    type NavVertex: Vertex<'a, Types=Self>;
    type NavEdge: Edge<'a, Types=Self>;
    type NavInEdgeCollection: InEdgeCollection<'a, Types=Self>;
    type NavOutEdgeCollection: OutEdgeCollection<'a, Types=Self>;
}

pub trait Vertex<'a>: Sized {
    type Types: NavTypes<'a, NavVertex=Self>;
    
    fn data(&self) -> &'a <<Self as Vertex<'a>>::Types as GraphTypes>::VertexData;

    fn out_edges(&self) -> <<Self as Vertex<'a>>::Types as NavTypes<'a>>::NavOutEdgeCollection;

    fn in_edges(&self) -> <<Self as Vertex<'a>>::Types as NavTypes<'a>>::NavInEdgeCollection;
}

pub trait Edge<'a>: Sized {
    type Types: NavTypes<'a, NavEdge=Self>;

    fn data(&self) -> &'a <<Self as Edge<'a>>::Types as GraphTypes>::EdgeData;

    fn source(&self) -> <<Self as Edge<'a>>::Types as NavTypes<'a>>::NavVertex;

    fn target(&self) -> <<Self as Edge<'a>>::Types as NavTypes<'a>>::NavVertex;
}

pub trait InEdgeCollection<'a>: Sized {
    type Types: NavTypes<'a, NavInEdgeCollection=Self>;
    type Iter: BoundedIterator<'a, Item=<<Self as InEdgeCollection<'a>>::Types as NavTypes<'a>>::NavEdge>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool { self.len() == 0 }

    fn target(&self) -> <<Self as InEdgeCollection<'a>>::Types as NavTypes<'a>>::NavVertex;

    fn iter(&self) -> Self::Iter;
}

pub trait OutEdgeCollection<'a>: Sized {
    type Types: NavTypes<'a, NavOutEdgeCollection=Self>;
    type Iter: BoundedIterator<'a, Item=<<Self as OutEdgeCollection<'a>>::Types as NavTypes<'a>>::NavEdge>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool { self.len() == 0 }

    fn source(&self) -> <<Self as OutEdgeCollection<'a>>::Types as NavTypes<'a>>::NavVertex;

    fn iter(&self) -> Self::Iter;
}
