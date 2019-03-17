use super::context::Context;
use super::dish_order::DishOrder;
use juniper::FieldResult;
use uuid::Uuid;

#[derive(Debug, ToSql, FromSql, GraphQLEnum)]
#[postgres(name = "customer_order_status")]
pub enum CustomerOrderStatus {
    Open,
    Closed,
    Done,
}

pub struct CustomerOrder {
    pub id: String,
    pub restaurant_id: String,
    pub dining_table_id: String,
    pub customer_id: String,
    pub status: CustomerOrderStatus,
}

graphql_object!(CustomerOrder: Context | &self | {
  field id() -> &str {
    self.id.as_str()
  }
  field restaurant_id() -> &str {
    self.restaurant_id.as_str()
  }
  field dining_table_id() -> &str {
    self.dining_table_id.as_str()
  }
  field customer_id() -> &str {
    self.customer_id.as_str()
  }
  field status() -> &CustomerOrderStatus {
    &self.status
  }
  field dishes(&executor) -> FieldResult<Vec<DishOrder>> {
    let conn = executor.context().pool.get()?;
    let customer_order_uuid = Uuid::parse_str(&self.id)?;
    let rows = conn.query("
      SELECT *
      FROM dish_order
      WHERE customer_order_id = $1
    ", &[&customer_order_uuid])?;
    let mut dishes = vec!();
    for row in &rows {
      let id: Uuid = row.get("id");
      let dish_id: Uuid = row.get("dish_id");
      let customer_order_id: Uuid = row.get("customer_order_id");
      let note: Option<String> = row.get("note");
      let quantity: i32 = row.get("quantity");
      let dish = DishOrder {
        id: id.hyphenated().to_string(),
        dish_id: dish_id.hyphenated().to_string(),
        customer_order_id: customer_order_id.hyphenated().to_string(),
        note: note,
        quantity: quantity,
      };
      dishes.push(dish);
    }
    Ok(dishes)
  }
});

#[derive(GraphQLInputObject)]
pub struct NewCustomerOrder {
    pub dining_table_id: String,
}
