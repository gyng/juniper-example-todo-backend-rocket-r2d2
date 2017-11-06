#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_cors;

mod db;
mod models;
mod schema;

use std::env;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use juniper::RootNode;
use r2d2_diesel::ConnectionManager;
use rocket::response::content;
use rocket::State;

type Schema = RootNode<'static, schema::QueryRoot, schema::MutationRoot>;

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<schema::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<schema::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    let cors_options = rocket_cors::Cors {
        allowed_origins: rocket_cors::AllowedOrigins::all(),
        allowed_methods: vec![rocket::http::Method::Get, rocket::http::Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: rocket_cors::AllowedHeaders::all(),
        allow_credentials: false,
        ..Default::default()
    };

    let query_root = schema::QueryRoot {};
    let mutation_root = schema::MutationRoot {};

    dotenv().expect("No .env file found");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let config = r2d2::Config::builder().pool_size(15).build();

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::new(config, manager).unwrap();

    rocket::ignite()
        .manage(schema::Context { pool: pool })
        .manage(Schema::new(query_root, mutation_root))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .attach(cors_options)
        .launch();
}
