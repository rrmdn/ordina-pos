#[derive(GraphQLObject)]
pub struct CustomerOrder {
    pub id: String,
    pub restaurant_id: String,
    pub dining_table_id: String,
    pub customer_id: String,
}

#[derive(GraphQLInputObject)]
pub struct NewCustomerOrder {
    pub dining_table_id: String,
}
