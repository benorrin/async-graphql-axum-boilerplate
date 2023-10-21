# Async-GraphQL Axum Boilerplate

> A simple boilerplate project implementing async-graphql and axum

[![GitHub tag](https://img.shields.io/github/tag/benorrin/async-graphql-axum-boilerplate?include_prereleases=&sort=semver&color=blue)](https://github.com/benorrin/async-graphql-axum-boilerplate/releases/)
[![License](https://img.shields.io/badge/License-MIT-blue)](#license)
[![issues - async-graphql-axum-boilerplate](https://img.shields.io/github/issues/benorrin/async-graphql-axum-boilerplate)](https://github.com/benorrin/async-graphql-axum-boilerplate/issues)

<hr />

A basic boilerplate project implementing async-graphql and axum in rust.

While trying to work with GraphQL in rust I found it to be a nightmare to get a basic project running. None of the documentation was particularly helpful.

This repo provides an oven ready implementation for you to expand for what purpose you may need.

Features:
- GraphQL implementation
- SQLx support (postgres)
- Basic Schema
- Basic Query
- Basic Mutation

## Usage

1. Clone the repo
2. Create a postgres database (schema shown below)
2. Add your own postgres DB connection string in main.rs
4. Run `cargo build`
5. Run the executable found in `target/debug` directory

## DB Schema

For the default implementation, a very basic schema is used:

**events** table

| id | name           |
|----|----------------|
| 1  | Birthday Party |
| 2  | Free Pizza     |
| 3  | Afternoon Nap  |

## License

Released under [MIT](/LICENSE) by [@benorrin](https://github.com/benorrin).