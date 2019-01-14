#[derive(GraphQLObject)]
pub struct DiningTable {
    pub id: String,
    pub name: String,
    pub restaurant_id: String,
}