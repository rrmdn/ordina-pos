#[derive(GraphQLObject)]
pub struct Partner {
    pub id: String,
    pub name: String,
    pub username: String,
    pub restaurant_id: String,
    pub picture: String,
    pub phone: String,
    pub email: String,
    pub is_active: bool,
}

#[derive(GraphQLInputObject)]
pub struct NewPartner {
    pub name: String,
    pub username: String,
    pub password: String,
    pub restaurant_id: String,
    pub picture: String,
    pub phone: String,
    pub email: String,
}

#[derive(GraphQLInputObject)]
pub struct PartnerSignIn {
    pub username: String,
    pub password: String,
}
