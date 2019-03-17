use juniper::{FieldError, FieldResult};
use uuid::Uuid;

use super::context::{Context, Roles};
use super::customer_order::{CustomerOrder, CustomerOrderStatus};
use super::dining_table::DiningTable;
use super::dish::Dish;
use super::restaurant::Restaurant;

pub struct Query;

graphql_object!(Query: Context |&self| {
    field request_customer_auth(&executor, phone: String) -> FieldResult<String> {
        executor.context().request_customer_auth(&phone, Roles::Customer)?;
        Ok("Requested".to_owned())
    }

    field current_customer_order(&executor) -> FieldResult<CustomerOrder> {
        let context = executor.context();
        context.authorize(Roles::Customer)?;
        let customer_id = context.get_client_id()?;
        let customer_uuid = Uuid::parse_str(&customer_id)?;
        let conn = context.pool.get()?;
        let customer_order_rows = conn.query("
            SELECT *
            FROM customer_order
            WHERE customer_id = $1 AND status = $2
        ", &[&customer_uuid, &CustomerOrderStatus::Open])?;
        if customer_order_rows.is_empty() {
            return Err(FieldError::new("Not found", graphql_value!({ "internal_error": "Not found" })));
        }

        let row = customer_order_rows.get(0);
        let id: Uuid = row.get("id");
        let restaurant_id: Uuid = row.get("restaurant_id");
        let dining_table_id: Uuid = row.get("dining_table_id");
        let customer_id: Uuid = row.get("customer_id");
        Ok(CustomerOrder {
            id: id.hyphenated().to_string(),
            restaurant_id: restaurant_id.hyphenated().to_string(),
            dining_table_id: dining_table_id.hyphenated().to_string(),
            customer_id: customer_id.hyphenated().to_string(),
            status: row.get("status"),
        })
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
