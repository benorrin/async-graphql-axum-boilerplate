use async_graphql::*;

// use crate::schema::types::*;

/// An account that owns `Individual` and `Business` profiles.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Event {
    /// The ID of the `Account`.
    pub id: i32,
    /// The email address of the `Account`.
    pub name: String,
}
