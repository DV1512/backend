pub mod delete;
pub mod download;
pub mod get;
pub mod upload;

use crate::models::{FileMetadata, FileMetadataMultiple};
use actix_web::{dev::HttpServiceFactory, web};
use utoipa::{openapi, path, OpenApi};

use delete::*;
use download::*;
use get::*;
use upload::*;

/// A simple file storage service.
/// Operations:
/// * Upload file
/// * Delete file and metadata
/// * Get metadata
/// * Get all metadata for user
/// * Download file
pub fn files_service() -> impl HttpServiceFactory {
    web::scope("/files")
        .service(upload_files)
        .service(get_file)
        .service(delete_file)
        .service(list_files)
        .service(download_file)
}

#[derive(OpenApi)]
#[openapi(
    paths(upload_files, get_file, delete_file, list_files, download_file),
    components(schemas(FileMetadata), responses(FileMetadata, FileMetadataMultiple))
)]
pub(crate) struct FilesApi;
