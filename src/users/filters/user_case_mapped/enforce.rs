// Copyright (c) 2020 Aravinth T M <realaravinth@batsense.net>.
// See the COPYRIGHT file at the top-level directory of this
// distribution

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//

use super::tables::USERNAME_CASE_MAPPED;
use crate::errors::{ServiceError, ServiceResult};
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

pub fn filter(target: &str) -> ServiceResult<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(USERNAME_CASE_MAPPED).unwrap();
    }
    let normalised = target.nfc().collect::<String>();
    if RE.is_match(&normalised) {
        Ok(normalised)
    } else {
        Err(ServiceError::CharError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let x = "\u{0065}";
        assert_eq!(filter(x).unwrap(), x)
    }
    #[test]
    fn test2() {
        let x = "\u{0063}";
        assert_eq!(filter(x).unwrap(), x)
    }
    #[test]
    fn test3() {
        let x = "\u{0000}";
        let y = match filter(x) {
            Ok(_) => false,
            Err(_x) => true,
        };
        assert!(y);
    }
    #[test]
    fn test4() {
        let x = "\u{0001}";
        let y = match filter(x) {
            Ok(_) => false,
            Err(_) => true,
        };
        assert!(y);
    }
}
