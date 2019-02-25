use juniper::{FieldError, FieldResult};
use uuid::Uuid;
use crypto::digest::Digest;

use super::context::{Context, Roles};
use super::dining_table::{DiningTable, NewDiningTable};
use super::dish::{Dish, NewDish};
use super::restaurant::{NewRestaurant, Restaurant};
use super::partner::{NewPartner, Partner, PartnerSignIn};

pub struct Mutation;

graphql_object!(Mutation: Context | &self | {
    field create_token_from_code(&executor, code: String) -> FieldResult<String> {
        if let Some(token) = executor.context().authenticate(&code).ok() {
            Ok(token)
        } else {
            Err(FieldError::new(
                "Code is no longer valid",
                graphql_value!({ "internal_error": "Code is no longer valid" })
            ))
        }
    }

    field create_restaurant(&executor, input: NewRestaurant) -> FieldResult<Restaurant> {
        let context = executor.context();
        context.authorize(Roles::Admin)?;
        let conn = context.pool.get()?;
        let id = Uuid::new_v4();
        let inserts = conn.execute("
            INSERT INTO restaurant (
                id,
                name,
                address,
                logo,
                cover,
                location_url
            ) VALUES ($1, $2, $3, $4, $5, $6)
        ", &[
            &id,
            &input.name,
            &input.address,
            &input.logo,
            &input.cover,
            &input.location_url
        ])?;
        let rows = conn.query("
            SELECT *
            FROM restaurant
            WHERE id = $1
        ", &[&id])?;

        let row = rows.get(0);
        let row_id: Uuid = row.get("id");
        Ok(Restaurant {
            id: row_id.hyphenated().to_string(),
            name: row.get("name"),
            address: row.get("address"),
            logo: row.get("logo"),
            cover: row.get("cover"),
            location_url: row.get("location_url"),
        })
    }

    field create_dining_table(&executor, input: NewDiningTable) -> FieldResult<DiningTable> {
        let context = executor.context();
        context.authorize(Roles::Partner)?;
        let conn = context.pool.get()?;
        let restaurant_id = context.get_partner_restaurant_id()?;
        let restaurant_uuid = Uuid::parse_str(&restaurant_id)?;
        let id = Uuid::new_v4();
        let inserts = conn.execute("
            INSERT INTO dining_table (
                id,
                name,
                restaurant_id
            ) VALUES ($1, $2, $3)
        ", &[
            &id,
            &input.name,
            &restaurant_uuid
        ])?;
        let rows = conn.query("
            SELECT *
            FROM dining_table
            WHERE id = $1
        ", &[&id])?;

        let row = rows.get(0);
        let row_id: Uuid = row.get("id");
        Ok(DiningTable {
            id: row_id.hyphenated().to_string(),
            name: row.get("name"),
            restaurant_id: restaurant_id,
        })
    }

    field create_dish(&executor, input: NewDish) -> FieldResult<Dish> {
        let context = executor.context();
        context.authorize(Roles::Partner)?;
        let restaurant_id = context.get_partner_restaurant_id()?;
        let restaurant_uuid = Uuid::parse_str(&restaurant_id)?;
        let conn = context.pool.get()?;
        let id = Uuid::new_v4();
        let inserts = conn.execute("
            INSERT INTO dish (
                id,
                name,
                price,
                description,
                restaurant_id
            ) VALUES ($1, $2, $3, $4, $5)
        ", &[
            &id,
            &input.name,
            &input.price,
            &input.description,
            &restaurant_uuid
        ])?;
        let rows = conn.query("
            SELECT *
            FROM dish
            WHERE id = $1
        ", &[&id])?;

        let row = rows.get(0);
        let row_id: Uuid = row.get("id");
        Ok(Dish {
            id: row_id.hyphenated().to_string(),
            name: row.get("name"),
            price: row.get("price"),
            description: row.get("description"),
            restaurant_id: restaurant_id,
        })
    }

    field partner_sign_up(&executor, input: NewPartner) -> FieldResult<Partner> {
        let context = executor.context();
        let conn = context.pool.get()?;
        let mut hasher = crypto::sha3::Sha3::sha3_224();
        hasher.input_str(&input.password);
        let hashed_password = hasher.result_str();
        let restaurant_id = Uuid::parse_str(&input.restaurant_id)?;
        let id = Uuid::new_v4();
        let inserts = conn.execute("
            INSERT INTO partner (
                id,
                name,
                username,
                hashed_password,
                email,
                phone,
                picture,
                restaurant_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ", &[
            &id,
            &input.name,
            &input.username,
            &hashed_password,
            &input.email,
            &input.phone,
            &input.picture,
            &restaurant_id,
        ])?;
        let rows = conn.query("
            SELECT *
            FROM partner
            WHERE id = $1
        ", &[&id])?;

        let row = rows.get(0);
        let row_id: Uuid = row.get("id");
        Ok(Partner {
            id: row_id.hyphenated().to_string(),
            name: row.get("name"),
            username: row.get("username"),
            email: row.get("email"),
            phone: row.get("phone"),
            picture: row.get("picture"),
            is_active: row.get("is_active"),
            restaurant_id: restaurant_id.hyphenated().to_string(),
        })
    }
    
    field partner_sign_in(&executor, input: PartnerSignIn) -> FieldResult<String> {
        let context = executor.context();
        let conn = context.pool.get()?;
        let mut hasher = crypto::sha3::Sha3::sha3_224();
        hasher.input_str(&input.password);
        let hashed_password = hasher.result_str();
        let rows = conn.query("
            SELECT *
            FROM partner
            WHERE username = $1 AND hashed_password = $2
        ", &[&input.username, &hashed_password])?;
        if rows.is_empty() {
            return Err(FieldError::new("Unauthenticated", graphql_value!({ "internal_error": "Unauthenticated" })));
        }
        let row = rows.get(0);
        let partner_id: Uuid = row.get("id");
        let token = context.create_partner_token(&partner_id.hyphenated().to_string())?;
        Ok(token)
    }
});
