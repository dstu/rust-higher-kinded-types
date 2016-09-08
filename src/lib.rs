//! This is a simple library designed to showcase a prospective design for a
//! family of zipper-like graph traversal traits. These traits may be
//! implemented over varying graph implementations (adjacency list, adjacency
//! matrix, lazily constructed infinite graph, actual zippers, etc.).
//!
//! Basic use of them goes something like this, assuming some type V that
//! implements nav_types::Vertex:
//!
//! ```rust,ignore
//! let vertex: V = ...;
//! // List children of vertex. vertex.out_edges() returns a type that
//! // implements nav_types::OutEdgeCollection.
//! for edge in vertex.out_edges().iter() {
//!   // edge implements nav_types::Edge.
//!   println!("vertex with data {:?} has child edge with data {:?}",
//!            vertex.data(), edge.data())
//! }
//! ```
//!
//! These traits are backed by a borrow of an arbitrary graph structure, and
//! Rust's ability to track lifetimes means that that may be done safely.

use std::iter::Iterator;

pub mod nav_types;
pub mod mut_types;

/// Base types that the graph defines a structure over.
pub trait GraphTypes: Sized {
    /// The type of data at vertices.
    type VertexData;
    /// The type of data at edges.
    type EdgeData;
}

/// When iterating over components of a graph structure, we need to give a name
/// to the borrow of the underlying graph. This trait does so.
pub trait BoundedIterator<'a>: Iterator { }
