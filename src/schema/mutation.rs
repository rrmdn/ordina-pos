use juniper::{FieldError, FieldResult};
use uuid::Uuid;

use super::context::Context;
use super::dining_table::{DiningTable, NewDiningTable};
use super::dish::{Dish, NewDish};
use super::restaurant::{NewRestaurant, Restaurant};

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
        let conn = executor.context().pool.get()?;
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
        let conn = executor.context().pool.get()?;
        let id = Uuid::new_v4();
        let restaurant_id = Uuid::parse_str(&input.restaurant_id)?;
        let inserts = conn.execute("
            INSERT INTO dining_table (
                id,
                name,
                restaurant_id
            ) VALUES ($1, $2, $3)
        ", &[
            &id,
            &input.name,
            &restaurant_id
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
            restaurant_id: input.restaurant_id,
        })
    }

    field create_dish(&executor, input: NewDish) -> FieldResult<Dish> {
        let conn = executor.context().pool.get()?;
        let id = Uuid::new_v4();
        let restaurant_id = Uuid::parse_str(&input.restaurant_id)?;
        let inserts = conn.execute("
            INSERT INTO dish (
                id,
                name,
                price,
                description,
                restaurant_id
            ) VALUES ($1, $2, $3)
        ", &[
            &id,
            &input.name,
            &input.price,
            &input.description,
            &restaurant_id
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
            restaurant_id: input.restaurant_id,
        })
    }
});
