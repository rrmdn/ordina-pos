use uuid::Uuid;
use juniper::{FieldError, FieldResult};

use super::context::{Context};
use super::restaurant::Restaurant;
use super::dining_table::DiningTable;

pub struct Query;

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }
    field restaurant(&executor, id: String) -> FieldResult<Restaurant> {
        let conn = executor.context().pool.get()?;
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
});
