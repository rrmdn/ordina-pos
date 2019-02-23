#[derive(GraphQLObject)]
pub struct Dish {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub restaurant_id: String,
}

#[derive(GraphQLInputObject)]
pub struct NewDish {
    pub name: String,
    pub description: String,
    pub price: i32,
}