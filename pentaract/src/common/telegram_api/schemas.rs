use serde::Deserialize;

use crate::common::types::ChatId;

#[derive(Deserialize)]
pub struct UploadBodySchema {
    pub result: UploadResultSchema,
}

#[derive(Deserialize)]
pub struct UploadResultSchema {
    pub document: UploadSchema,
}

#[derive(Deserialize)]
pub struct UploadSchema {
    pub file_id: String,
}

pub struct DownloadSchema {}
