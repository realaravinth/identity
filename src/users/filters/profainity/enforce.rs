use crate::errors::{ServiceError, ServiceResult};
use crate::RE_PROFAINITY;

pub fn beep(target: &str) -> ServiceResult<()> {
    if RE_PROFAINITY.is_match(&target) {
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
    fn profainity_ture3() {
        let x = "pundapayale";
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
