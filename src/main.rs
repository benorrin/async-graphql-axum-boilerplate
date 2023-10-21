use async_graphql::{http::GraphiQLSource, Context, EmptySubscription, Error, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use sqlx::PgPool;

use self::schema::{Mutation, Query};

mod schema;

/// A specialized result type for the application.
type Result<T> = std::result::Result<T, Error>;

/// Stores important state for the application.
#[derive(Clone)]
pub struct State {
    pub pool: PgPool,
}

/// Provides useful extension methods for [Context].
pub trait ContextExt {
    fn pool(&self) -> &PgPool;
}

impl ContextExt for Context<'_> {
    fn pool(&self) -> &PgPool {
        self.data_unchecked::<PgPool>()
    }
}

/// Handler function for requests made to the `GET /` endpoint.
/// Displays a GraphiQL interface for interacting with the GraphQL schema exposed by the application.
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("[DB_CONNECTION_STRING]")
        .await
        .expect("Failed to connect to database");

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(pool.clone())
        .finish();

    let app = Router::new()
    .route("/", get(graphiql)
    .with_state(State { pool })
    .post_service(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
