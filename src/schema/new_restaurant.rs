#[derive(GraphQLInputObject)]
pub struct NewRestaurant {
    pub name: String,
    pub address: String,
    pub logo: String,
    pub cover: String,
    pub location_url: String,
}