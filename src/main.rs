extern crate iron;
extern crate juniper_iron;
extern crate mount;
extern crate pretty_env_logger;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate juniper;
extern crate dotenv;
extern crate iron_json_response as ijr;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate uuid;
extern crate r2d2_redis;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;
extern crate crypto;
mod schema;

use std::env;
use std::error::Error;

use self::schema::context::context_factory;
use self::schema::mutation::Mutation;
use self::schema::query::Query;

use dotenv::dotenv;
use ijr::{JsonResponse, JsonResponseMiddleware};
use iron::prelude::*;
use iron::{AfterMiddleware};
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use logger::Logger;
use mount::Mount;

struct ResponseError;

impl AfterMiddleware for ResponseError {
    fn catch(&self, _req: &mut Request, err: IronError) -> IronResult<Response> {
        let error_description = err.description().to_string();
        let mut response = err.response;
        let error_message = json!({
            "data": serde_json::Value::Null,
            "errors": [{
                "message": error_description,
            }]
        });
        response.set_mut(JsonResponse::json(error_message));
        Ok(response)
    }
}

fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    let mut mount = Mount::new();
    let graphql_endpoint = GraphQLHandler::new(context_factory, Query, Mutation);
    let graphiql_endpoint = GraphiQLHandler::new("/graphql");
    let mut graphql_chain = Chain::new(graphql_endpoint);
    graphql_chain.link_after(ResponseError);
    mount.mount("/graphql", graphql_chain);
    mount.mount("/", graphiql_endpoint);
    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
    chain.link_after(JsonResponseMiddleware::new());
    let host = env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:4000".to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}
