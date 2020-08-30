use super::tables::PROFAINITY;
use crate::errors::{ServiceError, ServiceResult};
use regex::Regex;

pub fn beep(target: &str) -> ServiceResult<()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(PROFAINITY).unwrap();
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
    fn profainity_ture1() {
        let x = "fuck";
        let y = match beep(x) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert!(y);
    }

    #[test]
    fn profainity_ture2() {
        let x = "punda";
        let y = match beep(x) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert!(y);
    }

    #[test]
    fn profainity_false1() {
        let x = "hey";

        let y = match beep(x) {
            Ok(_) => true,
            Err(_) => false,
        };
        assert!(y);
    }

    #[test]
    fn profainity_false2() {
        let x = "rust";
        let y = match beep(x) {
            Ok(_) => true,
            Err(_) => false,
        };
        assert!(y);
    }
}
