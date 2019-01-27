use juniper::FieldResult;
use uuid::Uuid;

use super::context::Context;
use super::dining_table::DiningTable;

pub struct Restaurant {
  pub id: String,
  pub name: String,
  pub address: String,
  pub logo: String,
  pub cover: String,
  pub location_url: String,
}

graphql_object!(Restaurant: Context | &self | {
  field id() -> &str {
    self.id.as_str()
  }
  field name() -> &str {
    self.name.as_str()
  }
  field address() -> &str {
    self.address.as_str()
  }
  field logo() -> &str {
    self.logo.as_str()
  }
  field cover() -> &str {
    self.cover.as_str()
  }
  field location_url() -> &str {
    self.location_url.as_str()
  }
  field dining_table(&executor) -> FieldResult<Vec<DiningTable>> {
    let conn = executor.context().pool.get()?;
    let restaurant_id = Uuid::parse_str(&self.id)?;
    let rows = conn.query("
        SELECT *
        FROM dining_table
        WHERE restaurant_id = $1
    ", &[&restaurant_id])?;
    let mut dining_table_vec = vec!();
    for row in &rows {
      let row_id: Uuid = row.get("id");
      let restaurant_id: Uuid = row.get("restaurant_id");
      let dining_table = DiningTable {
        id: row_id.hyphenated().to_string(),
        name: row.get("name"),
        restaurant_id: restaurant_id.hyphenated().to_string(),
      };
      dining_table_vec.push(dining_table);
    }
    Ok(dining_table_vec)
  }
});

#[derive(GraphQLInputObject)]
pub struct NewRestaurant {
    pub name: String,
    pub address: String,
    pub logo: String,
    pub cover: String,
    pub location_url: String,
}
