use url::Url;

#[derive(Debug)]
pub struct ContentCommandResponse {

    path: String

}

impl ContentCommandResponse {

    pub fn get_path(&self) -> &String {
        &self.path
    }

}