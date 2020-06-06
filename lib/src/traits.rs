use crate::errors::Result;
use crate::models;
use crate::models::{EdgeQueryExt, VertexQueryExt};
use serde_json::value::Value as JsonValue;
use std::vec::Vec;

/// Specifies a datastore implementation.
///
/// # Errors
/// All methods may return an error if something unexpected happens - e.g.
/// if there was a problem connecting to the underlying database.
pub trait Datastore {
    type Trans: Transaction;

    /// Creates a new transaction.
    fn transaction(&self) -> Result<Self::Trans>;

    /// Bulk inserts many vertices, edges, and/or properties.
    ///
    /// # Arguments
    /// * `items`: The items to insert.
    fn bulk_insert<I>(&self, items: I) -> Result<models::BulkInsertResult>
    where
        I: Iterator<Item = models::BulkInsertItem>,
    {
        let trans = self.transaction()?;
        let mut id_range = None;

        for item in items {
            match item {
                models::BulkInsertItem::Vertex(t) => {
                    let id = trans.create_vertex(&t)?;
                    id_range = match id_range {
                        Some((start_id, _)) => Some((start_id, id)),
                        None => Some((id, id)),
                    };
                }
                models::BulkInsertItem::Edge(edge_key) => {
                    trans.create_edge(&edge_key)?;
                }
                models::BulkInsertItem::VertexProperty(id, name, value) => {
                    let query = models::SpecificVertexQuery::single(id).property(name);
                    trans.set_vertex_properties(query, &value)?;
                }
                models::BulkInsertItem::EdgeProperty(edge_key, name, value) => {
                    let query = models::SpecificEdgeQuery::single(edge_key).property(name);
                    trans.set_edge_properties(query, &value)?;
                }
            }
        }

        Ok(models::BulkInsertResult { id_range })
    }
}

/// Specifies a transaction implementation, which are returned by datastores.
/// All datastore manipulations are done through transactions. Despite the
/// name, different datastore implementations carry different guarantees.
/// Depending on the implementation, it may not be possible to rollback the
/// changes on error. See the documentation of individual implementations for
/// details. Transactions are automatically committed on drop. Transactions
/// should be designed to not fail on commit; i.e. errors should occur when a
/// method is actually called instead.
pub trait Transaction {
    /// Creates a new vertex. Returns the new vertex's ID.
    ///
    /// # Arguments
    /// * `t`: The type of the vertex to create.
    fn create_vertex(&self, t: &models::Type) -> Result<u64>;

    /// Gets a range of vertices specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn get_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<Vec<models::Vertex>>;

    /// Deletes existing vertices specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn delete_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<()>;

    /// Gets the number of vertices in the datastore..
    fn get_vertex_count(&self) -> Result<u64>;

    /// Creates a new edge. Returns whether the edge was successfully
    /// created - if this is false, it's because one of the specified vertices
    /// is missing.
    ///
    /// # Arguments
    /// * `key`: The edge to create.
    fn create_edge(&self, edge: &models::Edge) -> Result<bool>;

    /// Gets a range of edges specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn get_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<Vec<models::Edge>>;

    /// Deletes a set of edges specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn delete_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<()>;

    /// Gets the number of edges associated with a vertex.
    ///
    /// # Arguments
    /// * `id` - The id of the vertex.
    /// * `t` - Only get the count for a specified edge type.
    /// * `direction`: The direction of edges to get.
    fn get_edge_count(&self, id: u64, t: Option<&models::Type>, direction: models::EdgeDirection) -> Result<u64>;

    /// Gets vertex properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The property name.
    fn get_vertex_properties(&self, q: models::VertexPropertyQuery) -> Result<Vec<models::VertexProperty>>;

    /// Gets all vertex properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn get_all_vertex_properties<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<Vec<models::VertexProperties>>;

    /// Sets a vertex properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The property name.
    /// * `value` - The property value.
    fn set_vertex_properties(&self, q: models::VertexPropertyQuery, value: &JsonValue) -> Result<()>;

    /// Deletes vertex properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The property name.
    fn delete_vertex_properties(&self, q: models::VertexPropertyQuery) -> Result<()>;

    /// Gets edge properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The property name.
    fn get_edge_properties(&self, q: models::EdgePropertyQuery) -> Result<Vec<models::EdgeProperty>>;

    /// Gets all edge properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn get_all_edge_properties<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<Vec<models::EdgeProperties>>;

    /// Sets edge properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The property name.
    /// * `value` - The property value.
    fn set_edge_properties(&self, q: models::EdgePropertyQuery, value: &JsonValue) -> Result<()>;

    /// Deletes edge properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The property name.
    fn delete_edge_properties(&self, q: models::EdgePropertyQuery) -> Result<()>;
}
