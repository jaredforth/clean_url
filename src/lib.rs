// Copyright 2020 Jared Forth.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::utils::{parse_url, check_status};

pub mod utils;

/// Returns a URL
pub async fn return_url(url: String) -> Option<String> {
    match parse_url(url) {
        Some(u) => {
            check_status(u).await
        }
        None => {
            println!("Error parsing URL");
            None
        }
    }
}