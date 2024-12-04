use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
}
