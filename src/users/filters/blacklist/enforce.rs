use super::tables::BLACKLIST;
use crate::errors::{ServiceError, ServiceResult};
use regex::Regex;

pub fn forbidden(target: &str) -> ServiceResult<()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(BLACKLIST).unwrap();
    }
    if RE.is_match(&target) {
        Err(ServiceError::CharError)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forbidden_ture1() {
        let x = "zlib";

        let y = match forbidden(x) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert!(y);
    }

    #[test]
    fn forbidden_ture2() {
        let x = ".htaccess";
        let y = match forbidden(x) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert!(y);
    }

    #[test]
    fn forbidden_ture3() {
        let x = ".htaccess_yolo";
        let y = match forbidden(x) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert!(y);
    }

    #[test]
    fn forbidden_false1() {
        let x = "hey";

        let y = match forbidden(x) {
            Ok(_) => true,
            Err(_) => false,
        };
        assert!(y);
    }

    #[test]
    fn forbidden_false2() {
        let x = "rust";
        let y = match forbidden(x) {
            Ok(_) => true,
            Err(_) => false,
        };
        assert!(y);
    }
}
