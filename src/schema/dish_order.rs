use super::context::Context;
use super::dish::Dish;
use juniper::{FieldError, FieldResult};
use uuid::Uuid;

pub struct DishOrder {
    pub id: String,
    pub dish_id: String,
    pub customer_order_id: String,
    pub note: Option<String>,
    pub quantity: i32,
}

graphql_object!(DishOrder: Context | &self | {
  field id() -> &str {
    self.id.as_str()
  }
  field dish_id() -> &str {
    self.dish_id.as_str()
  }
  field customer_order_id() -> &str {
    self.customer_order_id.as_str()
  }
  field note() -> &Option<String> {
    &self.note
  }
  field quantity() -> i32 {
    self.quantity
  }
  field dish(&executor) -> FieldResult<Dish> {
    let conn = executor.context().pool.get()?;
    let dish_uuid = Uuid::parse_str(&self.dish_id)?;
    let rows = conn.query("
      SELECT *
      FROM dish
      WHERE id = $1
    ", &[&dish_uuid])?;
    if rows.is_empty() {
      return Err(FieldError::new("Dish does not exist", graphql_value!({ "internal_error": "Dish does not exist" })));
    }
    let row = rows.get(0);
    let restaurant_id: Uuid = row.get("restaurant_id");
    Ok(Dish {
      id: dish_uuid.hyphenated().to_string(),
      restaurant_id: restaurant_id.hyphenated().to_string(),
      name: row.get("name"),
      description: row.get("description"),
      price: row.get("price"),
    })
  }
});

#[derive(GraphQLInputObject)]
pub struct NewDishOrder {
    pub dish_id: String,
    pub customer_order_id: String,
    pub note: Option<String>,
    pub quantity: i32,
}
