#[derive(GraphQLObject)]
pub struct DiningTable {
    pub id: String,
    pub name: String,
    pub restaurant_id: String,
}

#[derive(GraphQLInputObject)]
pub struct NewDiningTable {
    pub name: String,
}
