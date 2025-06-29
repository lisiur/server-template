use entity::uploads;

use crate::impl_service;

pub mod create_upload;
pub mod query_uploads;

impl_service!(UploadService, uploads::Entity);
