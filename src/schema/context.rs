use iron::headers::{Authorization, Bearer};
use iron::prelude::*;
use juniper::{FieldError, FieldResult};
use jwt::{decode, encode, Header, TokenData, Validation};
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use r2d2_redis::redis::Commands;
use r2d2_redis::{r2d2, RedisConnectionManager};
use rand::{thread_rng, Rng};
use std::env;
use uuid::Uuid;
use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: i64,
}

#[derive(GraphQLInputObject)]
pub struct AuthInfo {
    pub token: String,
}

pub struct Context {
    pub pool: Pool<PostgresConnectionManager>,
    pub redis_pool: Pool<RedisConnectionManager>,
    pub claims: Option<TokenData<Claims>>,
}

impl Context {
    pub fn authenticate(&self, code: &String) -> FieldResult<String> {
        let redis = self.redis_pool.get()?;
        let key = env::var("JWT_AUTH_SECRET")?;
        let client_id = redis.get(code)?;
        let claims = Claims {
            sub: client_id,
            company: "RRMDN".to_owned(),
            exp: Utc::now().timestamp() + (60 * 60 * 24),
        };
        let _: () = redis.del(code)?;
        let token = encode(&Header::default(), &claims, key.as_ref())?;
        Ok(token)
    }
    pub fn request_auth(&self, phone: &String) -> FieldResult<()> {
        let redis = self.redis_pool.get()?;
        let db = self.pool.get()?;
        let rows = db.query(
            "
            SELECT *
            FROM customer
            WHERE phone = $1
        ",
            &[&phone],
        )?;
        if rows.is_empty() {
            return Err(FieldError::new(
                "Not found",
                graphql_value!({ "internal_error": "Not found" }),
            ));
        }
        let row = rows.get(0);
        let customer_id: Uuid = row.get("id");
        let code = thread_rng().gen_range(100000, 999999);
        let _: () = redis.set_ex(code, customer_id.hyphenated().to_string(), 3600)?;
        Ok(())
    }
}

impl juniper::Context for Context {}

pub fn context_factory(req: &mut Request) -> IronResult<Context> {
    let auth_header = req.headers.get::<Authorization<Bearer>>();
    let key = env::var("JWT_AUTH_SECRET").unwrap();
    let claims = {
        if let Some(bearer) = auth_header {
            let validation = Validation::default();
            match decode::<Claims>(&bearer.0.token, key.as_ref(), &validation) {
                Err(_) => None,
                Ok(t) => Some(t),
            }
        } else {
            None
        }
    };
    let manager = PostgresConnectionManager::new(
        env::var("POSTGRES_CONNECTION_STRING").unwrap(),
        TlsMode::None,
    )
    .unwrap();
    let redis_manager =
        RedisConnectionManager::new(env::var("REDIS_CONNECTION_STRING").unwrap().as_str()).unwrap();
    Ok(Context {
        pool: Pool::new(manager).unwrap(),
        redis_pool: Pool::new(redis_manager).unwrap(),
        claims,
    })
}
