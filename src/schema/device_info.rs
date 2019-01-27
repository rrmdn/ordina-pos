#[derive(GraphQLObject)]
pub struct DeviceInfo {
    pub id: String,
    pub customer_id: Option<String>,
    pub user_agent: Option<String>,
}
