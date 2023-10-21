use async_graphql::*;

use self::event::{EventMutation, EventQuery};

mod event;
mod types;

/// The root query type.
#[derive(Default, MergedObject)]
pub struct Query(EventQuery);

/// The root mutation type.
#[derive(Default, MergedObject)]
pub struct Mutation(EventMutation);
