extern crate iron;
extern crate juniper_iron;
extern crate mount;
extern crate pretty_env_logger;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate juniper;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate uuid;
extern crate iron_json_response as ijr;
extern crate dotenv;

mod schema;

use std::env;
use std::fmt::{self, Debug, Formatter};
use std::error::Error;

use self::schema::mutation::{Mutation};
use self::schema::query::{Query};
use self::schema::context::{context_factory};

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware};
use iron::headers::{Bearer, Authorization};
use iron::status::Status;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use logger::Logger;
use mount::Mount;
use ijr::{JsonResponse, JsonResponseMiddleware};
use dotenv::dotenv;

#[derive(Debug)]
struct StringError(String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str {
        &*self.0
    }
}

struct Authentication;

impl BeforeMiddleware for Authentication {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let auth = req.headers.get::<Authorization<Bearer>>();
        if let Some(bearer) = auth {
            if bearer.0.token == "TEST" {
                return Ok(());
            }
        }
        Err(IronError::new(StringError("Authorization Required".to_string()), Status::Unauthorized))
    }
}

impl AfterMiddleware for Authentication {
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
    mount.mount("/graphql", graphql_endpoint);
    mount.mount("/", graphiql_endpoint);
    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_before(Authentication);
    chain.link_after(logger_after);
    chain.link_after(Authentication);
    chain.link_after(JsonResponseMiddleware::new());
    let host = env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:4000".to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}
