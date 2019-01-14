#[derive(GraphQLInputObject)]
pub struct NewDiningTable {
    pub name: String,
    pub restaurant_id: String,
}