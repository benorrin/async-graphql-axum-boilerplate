use async_graphql::{Context, Result, Object, ComplexObject};
use sqlx::{query_file_as};

use crate::{schema::types::*, ContextExt};

#[ComplexObject]
impl Event {
    /// The token used for associating purchases with the `Event`.
    async fn events(&self, context: &Context<'_>) -> Result<Vec<Event>> {
        let pool = context.pool();

        let events = query_file_as!(Event, "queries/get-events.sql")
            .fetch_all(pool)
            .await?;

        Ok(events)
    }
}

#[derive(Default)]
pub struct EventQuery;

#[Object]
impl EventQuery {

    // Get all 'events'
    async fn events(&self, context: &Context<'_>) -> Result<Vec<Event>> {
        let pool = context.pool();

        let events = query_file_as!(Event, "queries/get-events.sql")
            .fetch_all(pool)
            .await?;

        Ok(events)
    }
}

#[derive(Default)]
pub struct EventMutation;

#[Object]
impl EventMutation {
    
    /// Create a new `Event`.
    async fn create_event(
        &self,
        _context: &Context<'_>,
        // #[graphql(desc = "Parameters for `createEvent`.")] input: CreateEventInput,
    ) -> Result<String> {
        Ok("Kebab".to_string())
    }
}
