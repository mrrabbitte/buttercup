use actix_files::Files;
use std::fs::File;
use std::io;
pub mod naming;

pub struct FilesService {

    root_path: &'static str,
    root_dir: &'static str

}

impl FilesService {

    pub fn get_files(&self) -> Files {
        Files::new(self.root_path, self.root_dir)
    }

    pub fn create_new(&self) -> io::Result<File> {
        File::create(self.root_dir)
    }

}
