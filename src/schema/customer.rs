#[derive(GraphQLObject)]
pub struct Customer {
    pub id: String,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}
