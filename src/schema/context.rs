use chrono::prelude::*;
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Roles {
    Admin,
    Customer,
    Partner,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: i64,
    pub role: Roles,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCode {
    client_id: String,
    role: Roles,
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
        let auth_code_string: String = redis.get(code)?;
        let auth_code: AuthCode = serde_json::from_str(&auth_code_string)?;
        let claims = Claims {
            sub: auth_code.client_id,
            company: "RRMDN".to_owned(),
            exp: Utc::now().timestamp() + (60 * 60 * 24),
            role: auth_code.role,
        };
        let _: () = redis.del(code)?;
        let token = encode(&Header::default(), &claims, key.as_ref())?;
        Ok(token)
    }
    pub fn authorize(&self, role: Roles) -> FieldResult<()> {
        if let Some(claims) = &self.claims {
            if claims.claims.role == role {
                return Ok(());
            } else {
                return Err(FieldError::new(
                    "Unauthorized",
                    graphql_value!({ "internal_error": "Unauthorized" }),
                ));
            }
        }
        Err(FieldError::new(
            "Unauthenticated",
            graphql_value!({ "internal_error": "Unauthenticated" }),
        ))
    }
    pub fn get_client_id(&self) -> FieldResult<&String> {
        if let Some(claims) = &self.claims {
            return Ok(&claims.claims.sub);
        }
        Err(FieldError::new(
            "Unauthenticated",
            graphql_value!({ "internal_error": "Unauthenticated" }),
        ))
    }
    pub fn get_partner_restaurant_id(&self) -> FieldResult<String> {
        self.authorize(Roles::Partner)?;
        let partner_id = self.get_client_id()?;
        let partner_uuid = Uuid::parse_str(&partner_id)?;
        let db = self.pool.get()?;
        let rows = db.query(
            "
            SELECT restaurant_id
            FROM customer
            WHERE id = $1
        ",
            &[&partner_uuid],
        )?;
        if rows.is_empty() {
            return Err(FieldError::new(
                "Not found",
                graphql_value!({ "internal_error": "Not found" }),
            ));
        }
        let row = rows.get(0);
        let restaurant_id: Uuid = row.get("restaurant_id");
        Ok(restaurant_id.hyphenated().to_string())
    }
    pub fn request_customer_auth(&self, phone: &String, role: Roles) -> FieldResult<()> {
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
        let auth_code = AuthCode {
            client_id: customer_id.hyphenated().to_string(),
            role: role,
        };
        let auth_code_string = serde_json::to_string(&auth_code)?;
        let _: () = redis.set_ex(code, auth_code_string, 3600)?;
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
