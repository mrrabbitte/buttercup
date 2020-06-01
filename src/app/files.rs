use actix_files::Files;
use std::fs::File;
use std::io;
use crate::app::files::path::{FilesPathService, FilesPathServiceError};
use std::io::Error;
use serde::{Serialize, Deserialize};
pub mod path;

#[derive(Debug, Clone)]
pub struct FileService {

    root_path: &'static str,
    root_dir: &'static str,
    files_path_service: FilesPathService

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilesServiceError {

    FilesPathServiceError(FilesPathServiceError),
    FileCreationError(String)
}

pub struct FileResponse {

    file: File,
    external_path: String

}

impl FileResponse {

    pub fn new(file: File,
               external_path: String) -> FileResponse {
        FileResponse {
            file,
            external_path
        }
    }

    pub fn get_file(&mut self) -> &mut File {
        &mut self.file
    }

    pub fn get_external_path(&self) -> &String {
        &self.external_path
    }

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

    pub fn create_new_html(&self, tenant_id: &String) -> Result<FileResponse, FilesServiceError> {
        FileService::create(self.files_path_service.new_html(tenant_id))
    }

    pub fn create_new_mp4(&self, tenant_id: &String) -> Result<FileResponse, FilesServiceError> {
        FileService::create(self.files_path_service.new_mp4(tenant_id))
    }

    fn create(result: Result<String, FilesPathServiceError>)
        -> Result<FileResponse, FilesServiceError> {
        match result {
            Ok(path) => match File::create(&path) {
                Ok(file) => Result::Ok(FileResponse::new(file, path)),
                Err(err) => Result::Err(
                    FilesServiceError::FileCreationError(
                        format!("Path: {}, Error: {}", path, err))),
            },
            Err(err) => Result::Err(FilesServiceError::FilesPathServiceError(err)),
        }
    }

}
