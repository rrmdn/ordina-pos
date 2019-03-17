use crypto::digest::Digest;
use juniper::{FieldError, FieldResult};
use uuid::Uuid;

use super::context::{Context, Roles};
use super::customer_order::{CustomerOrder, NewCustomerOrder};
use super::dining_table::{DiningTable, NewDiningTable};
use super::dish::{Dish, NewDish};
use super::partner::{NewPartner, Partner, PartnerSignIn};
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

    field create_anonymous_customer_token(&executor) -> FieldResult<String> {
        executor.context().create_anonymous_customer_token()
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

    field create_customer_order(&executor, input: NewCustomerOrder) -> FieldResult<CustomerOrder> {
        let context = executor.context();
        context.authorize(Roles::Customer)?;
        let customer_id = context.get_client_id()?;
        let customer_uuid = Uuid::parse_str(&customer_id)?;
        let dining_table_uuid = Uuid::parse_str(&input.dining_table_id)?;
        let conn = context.pool.get()?;

        let dining_table_rows = conn.query("
            SELECT restaurant_id
            FROM dining_table
            WHERE id = $1", &[&dining_table_uuid])?;
        if dining_table_rows.is_empty() {
            return Err(FieldError::new("Notfound", graphql_value!({"external_error": "Dining table does not exist"})));
        }

        let dining_table_row = dining_table_rows.get(0);
        let restaurant_uuid: Uuid = dining_table_row.get("restaurant_id");
        let customer_order_uuid = Uuid::new_v4();

        let inserts = conn.query("
            INSERT INTO customer_order (
                id,
                restaurant_id,
                dining_table_id,
                customer_id
            ) VALUES ($1, $2, $3, $4)
        ", &[&customer_order_uuid, &restaurant_uuid, &dining_table_uuid, &customer_uuid])?;

        Ok(CustomerOrder {
            id: customer_order_uuid.hyphenated().to_string(),
            restaurant_id: restaurant_uuid.hyphenated().to_string(),
            dining_table_id: dining_table_uuid.hyphenated().to_string(),
            customer_id: customer_uuid.hyphenated().to_string(),
        })
    }
});
