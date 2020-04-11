use fast_chemail;
use fast_chemail::ParseError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub struct Email {

    value: String

}

impl Email {

    pub fn new(value: &str) -> Result<Email, ParseError> {
        match fast_chemail::parse_email(value) {
            Ok(_) => Result::Ok(
                Email {
                    value: value.to_string()
                }),
            Err(err) => Result::Err(err),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_valid() {
        let ok_emails = ["email@example.com",
            "firstname.lastname@example.com",
            "email@subdomain.example.com",
            "firstname+lastname@example.com",
            "email@123.123.123.123",
            "1234567890@example.com",
            "email@example-one.com",
            "_______@example.com",
            "email@example.name",
            "email@example.museum",
            "email@example.co.jp",
            "firstname-lastname@example.com"];
        for ok_email in &ok_emails {
            assert_eq!(true, Email::new(ok_email).is_ok(), "{}", ok_email);
        }
    }

    #[test]
    fn test_basic_invalid() {
        let err_emails = ["plainaddress",
            "#@%^%#$@#$@#.com",
            "@example.com",
            "email.example.com",
            "email@example@example.com",
            ".email@example.com",
            "email.@example.com",
            "email..email@example.com",
            "あいうえお@example.com",
            "email@example.com (Joe Smith)",
            "email@example",
            "email@-example.com",
            "email@example..com",
            "Abc..123@example.com"];
        for err_email in &err_emails {
            assert_eq!(false, Email::new(err_email).is_ok(), "{}", err_email);
        }
    }

}