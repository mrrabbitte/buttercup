use ring::rand::{SystemRandom, SecureRandom};
use base64::URL_SAFE_NO_PAD;
use ring::error::Unspecified;
use std::fmt::Write;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilesPathServiceError {

    TenantIdNotFound(String)

}

#[derive(Debug, Clone)]
pub struct FilesPathService {

    name_generator: NameGenerator,
    base_path: &'static str,
    tenant_paths: HashMap<String, String>

}

impl FilesPathService {

    pub fn new(base_path: &'static str,
               tenant_paths: HashMap<String, String>) -> FilesPathService {
        FilesPathService {
            name_generator: NameGenerator::new(),
            base_path,
            tenant_paths
        }
    }

    pub fn new_html(&self,
                    tenant_id: &String) -> Result<String, FilesPathServiceError> {
        self.get_path(tenant_id, ".html")
    }

    pub fn new_mp4(&self,
                    tenant_id: &String) -> Result<String, FilesPathServiceError> {
        self.get_path(tenant_id, ".mp4")
    }

    fn get_path(&self,
                tenant_id: &String,
                extension: &str) -> Result<String, FilesPathServiceError> {
        match self.tenant_paths.get(tenant_id) {
            None => Result::Err(FilesPathServiceError::TenantIdNotFound(tenant_id.clone())),
            Some(tenant_path) => Result::Ok(self.do_get_path(tenant_path, extension)),
        }
    }

    fn do_get_path(&self,
                   tenant_path: &String,
                   extension: &str) -> String {
        let mut result = String::new();
        result.write_str(&self.base_path).expect("This should never fail.");
        result.write_str(tenant_path).expect("This should never fail.");
        let gen_result = self.name_generator.generate_random(&mut result);
        if gen_result.is_err() {
            panic!("Generate random operation should not fail: {:?}", gen_result);
        }
        result.write_str(extension).expect("This should never fail.");
        result
    }

}

#[derive(Debug, Clone)]
struct NameGenerator {

    secure_random: SystemRandom

}

impl NameGenerator {

    fn new() -> NameGenerator {
        NameGenerator {
            secure_random: SystemRandom::new()
        }
    }

    fn generate_random(&self,
                       target: &mut String) -> Result<(), ()> {
        let mut random_bytes = [0; 20];
        match self.secure_random.fill(&mut random_bytes) {
            Ok(_) => {
                base64::encode_config_buf(random_bytes, URL_SAFE_NO_PAD, target);
                Result::Ok(())
            },
            Err(_) => Result::Err(()),
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    const TENANT_ID: &str = "tenantid1";
    const BASE_PATH: &str = "basepath/";
    const TENANT_PATH: &str = "tenantid1/";

    #[test]
    fn test_happy_path_html() {
        let mut tenant_paths = HashMap::new();
        tenant_paths.insert(TENANT_ID.to_owned(), TENANT_PATH.to_owned());
        let service = FilesPathService::new(BASE_PATH, tenant_paths);
        let result = service.new_html(&TENANT_ID.to_owned());
        assert_eq!(true, result.is_ok());
        let name = result.unwrap();
        assert_eq!(true, name.starts_with(BASE_PATH));
        assert_eq!(true, name.contains(TENANT_PATH));
        assert_eq!(true, name.contains(".html"));
    }

    #[test]
    fn test_happy_path_mp4() {
        let mut tenant_paths = HashMap::new();
        tenant_paths.insert(TENANT_ID.to_owned(), TENANT_PATH.to_owned());
        let service = FilesPathService::new(BASE_PATH, tenant_paths);
        let result = service.new_mp4(&TENANT_ID.to_owned());
        assert_eq!(true, result.is_ok());
        let name = result.unwrap();
        assert_eq!(true, name.starts_with(BASE_PATH));
        assert_eq!(true, name.contains(TENANT_PATH));
        assert_eq!(true, name.contains(".mp4"));
    }

    #[test]
    fn test_unknown_tenant() {
        let service = FilesPathService::new(BASE_PATH, HashMap::new());
        let result = service.new_html(&TENANT_ID.to_owned());
        assert_eq!(true, result.is_err());
    }

}