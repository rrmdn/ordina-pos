
use juniper::{FieldResult};
use uuid::Uuid;

use super::context::Context;
use super::query::Restaurant;

#[derive(GraphQLInputObject)]
pub struct NewRestaurant {
    pub name: String,
    pub address: String,
    pub logo: String,
    pub cover: String,
    pub location_url: String,
}

pub struct Mutation;

graphql_object!(Mutation: Context | &self | {
    field apiVersion() -> &str {
        "1.0"
    }
    field createRestaurant(&executor, new_restaurant: NewRestaurant) -> FieldResult<Restaurant> {
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
            &new_restaurant.name,
            &new_restaurant.address,
            &new_restaurant.logo,
            &new_restaurant.cover,
            &new_restaurant.location_url
        ])?;
        let rows = conn.query("
            SELECT *
            FROM restaurant
            WHERE id = $1
        ", &[&id])?;

        let row = rows.get(0);
        let restaurant_id: Uuid = row.get("id");
        Ok(Restaurant {
            id: restaurant_id.hyphenated().to_string(),
            name: row.get("name"),
            address: row.get("address"),
            logo: row.get("logo"),
            cover: row.get("cover"),
            location_url: row.get("location_url"),
        })
    }
});