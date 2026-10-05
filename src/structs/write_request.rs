use sonic_rs::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteRequest {
    pub timestamp: u64,
    pub app_name: String,
    pub data: String,
}
