use crate::biz::model::Model;

pub struct UploadHistory {
    pub hid: i64,
    pub key: String,
    pub url: String,
    pub description: Option<String>,
    pub status: i8,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub deleted: bool,
}
impl UploadHistory {
    pub fn new(key: String, url: String) -> Self {
        Self {
            hid: 0,
            key,
            url,
            description: None,
            status: 0,
            created_time: chrono::Utc::now(),
            deleted: false,
        }
    }
}

impl Model<UploadHistory,i64> for UploadHistory {

}