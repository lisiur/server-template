use entity::upload_chunks;

use crate::impl_service;

impl_service!(UploadChunkService, upload_chunks::Entity);
