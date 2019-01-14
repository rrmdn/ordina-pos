
use juniper::{FieldResult};
use uuid::Uuid;

use super::context::Context;
use super::restaurant::Restaurant;
use super::new_restaurant::NewRestaurant;
use super::dining_table::DiningTable;
use super::new_dining_table::NewDiningTable;

pub struct Mutation;

graphql_object!(Mutation: Context | &self | {
    field apiVersion() -> &str {
        "1.0"
    }
    field createRestaurant(&executor, input: NewRestaurant) -> FieldResult<Restaurant> {
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
    field createDiningTable(&executor, input: NewDiningTable) -> FieldResult<DiningTable> {
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
});