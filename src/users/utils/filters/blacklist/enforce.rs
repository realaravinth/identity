/*
* Copyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::errors::{ServiceError, ServiceResult};
use crate::RE_BLACKLIST;

pub fn forbidden(target: &str) -> ServiceResult<()> {
    if RE_BLACKLIST.is_match(&target) {
        Err(ServiceError::UsernameError)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forbidden() {
        let illegal = "zlib";
        let legal = "rust";
        let illegal2 = ".htaccess_yolo";

        assert_eq!(forbidden(legal), Ok(()));
        assert_eq!(forbidden(illegal), Err(ServiceError::UsernameError));
        assert_eq!(forbidden(illegal2), Err(ServiceError::UsernameError));
    }
}
