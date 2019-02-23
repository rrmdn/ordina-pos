use uuid::Uuid;
use juniper::{FieldError, FieldResult};

use super::context::{Context, Roles};
use super::restaurant::Restaurant;
use super::dining_table::DiningTable;
use super::dish::Dish;

pub struct Query;

graphql_object!(Query: Context |&self| {
    field request_customer_auth(&executor, phone: String) -> FieldResult<String> {
        executor.context().request_customer_auth(&phone, Roles::Customer)?;
        Ok("Requested".to_owned())
    }
    field restaurant(&executor, id: String) -> FieldResult<Restaurant> {
        let context = executor.context();
        let conn = context.pool.get()?;
        let parsed_id = Uuid::parse_str(&id)?;
        let rows = conn.query("
            SELECT *
            FROM restaurant
            WHERE id = $1
        ", &[&parsed_id])?;
        if rows.is_empty() {
            return Err(FieldError::new("Not found", graphql_value!({ "internal_error": "Not found" })));
        }
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

    field dining_table(&executor, id: String) -> FieldResult<DiningTable> {
        let conn = executor.context().pool.get()?;
        let parsed_id = Uuid::parse_str(&id)?;
        let rows = conn.query("
            SELECT *
            FROM dining_table
            WHERE id = $1
        ", &[&parsed_id])?;
        if rows.is_empty() {
            return Err(FieldError::new("Not found", graphql_value!({ "internal_error": "Not found" })));
        }
        let row = rows.get(0);
        let row_id: Uuid = row.get("id");
        Ok(DiningTable {
            id: row_id.hyphenated().to_string(),
            name: row.get("name"),
            restaurant_id: row.get("restaurant_id"),
        })
    }

    field dish(&executor, id: String) -> FieldResult<Dish> {
        let conn = executor.context().pool.get()?;
        let parsed_id = Uuid::parse_str(&id)?;
        let rows = conn.query("
            SELECT *
            FROM dish
            WHERE id = $1
        ", &[&parsed_id])?;
        if rows.is_empty() {
            return Err(FieldError::new("Not found", graphql_value!({ "internal_error": "Not found" })));
        }
        let row = rows.get(0);
        let row_id: Uuid = row.get("id");
        Ok(Dish {
            id: row_id.hyphenated().to_string(),
            name: row.get("name"),
            price: row.get("price"),
            description: row.get("description"),
            restaurant_id: row.get("restaurant_id"),
        })
    }
});
