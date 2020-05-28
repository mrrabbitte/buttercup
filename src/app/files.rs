use actix_files::Files;
use std::fs::File;
use std::io;
use crate::app::files::path::{FilesPathService, FilesPathServiceError};
use std::io::Error;

pub mod path;

pub struct FileService {

    root_path: &'static str,
    root_dir: &'static str,
    files_path_service: FilesPathService

}

pub enum FilesServiceError {

    FilesPathServiceError(FilesPathServiceError),
    FileCreationError(String)
}

impl FileService {

    pub fn new(root_path: &'static str,
               root_dir: &'static str,
               files_path_service: FilesPathService) -> FileService {
        FileService {
            root_path,
            root_dir,
            files_path_service
        }
    }

    pub fn get_files(&self) -> Files {
        Files::new(self.root_path, self.root_dir)
    }

    pub fn create_new_html(&self, tenant_id: &String) -> Result<File, FilesServiceError> {
        FileService::create(self.files_path_service.new_html(tenant_id))
    }

    pub fn create_new_mp4(&self, tenant_id: &String) -> Result<File, FilesServiceError> {
        FileService::create(self.files_path_service.new_mp4(tenant_id))
    }

    fn create(result: Result<String, FilesPathServiceError>) -> Result<File, FilesServiceError> {
        match result {
            Ok(path) => match File::create(path) {
                Ok(file) => Result::Ok(file),
                Err(err) => Result::Err(
                    FilesServiceError::FileCreationError(
                        format!("{}", err))),
            },
            Err(err) => Result::Err(FilesServiceError::FilesPathServiceError(err)),
        }
    }

}
