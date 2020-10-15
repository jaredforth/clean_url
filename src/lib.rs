// Copyright 2020 Jared Forth.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Gets the best version of a URL.
//!
//! **clean_url** takes a URL, formats it correctly, and returns the version of the URL
//! that returns a 200 series status code.

#[macro_use] extern crate lazy_static;

use crate::utils::{parse_url, check_status};

pub mod utils;

/// Returns a clean URL that has a 200 series status code
///
/// ## Usage:
///
/// ```
/// use clean_url::return_url;
/// use tokio_test::block_on;
///
/// assert_eq!(block_on(return_url(String::from("https://httpbin.org/status/200"), false)), Some(String::from("https://httpbin.org/status/200")));
/// assert_eq!(block_on(return_url(String::from("example.com"), false)), Some(String::from("http://example.com/")));
/// assert_eq!(block_on(return_url(String::from("example.com"), true)), Some(String::from("https://example.com/")));
/// ```
pub async fn return_url(url: String, is_secure: bool) -> Option<String> {
    match parse_url(url, is_secure) {
        Some(u) => {
            check_status(u).await
        }
        None => {
            println!("Error parsing URL");
            None
        }
    }
}