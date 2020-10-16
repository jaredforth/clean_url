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

use crate::utils::{parse_url, check_status, swap_www};

pub mod utils;

/// Returns a clean URL that has a 200 series status code
///
/// ## Usage:
///
/// ```
/// use clean_url::return_url;
/// use tokio_test::block_on;
///
/// assert_eq!(block_on(return_url(String::from("httpbin.org/status/200"))), Some(String::from("https://httpbin.org/status/200")));
/// assert_eq!(block_on(return_url(String::from("example.com"))), Some(String::from("https://example.com/")));
/// ```
pub async fn return_url(url: String) -> Option<String> {
    // Parse URL
    // check http + www
    // check just http
    // if !200,
    // check https + www
    // check just https
    match parse_url(url.clone(), true) {
        // `u` here will be the original URL + https://
        Some(u) => {
            // Check original version of URL
            match check_status(&u).await {
                // If success, return
                Some(s) => Some(s),
                None => {
                    // Check next version of URL (either with or without www)
                    match check_status(&swap_www(&u).await).await {
                        // If success, return
                        Some(s2) => Some(s2),
                        None => {
                            // Check http version
                            match parse_url(url, false) {
                                // `u2` here will be the original URL + http://
                                Some(u2) => {
                                    // Check first version
                                    match check_status(&u2).await {
                                        // If success, return
                                        Some(s3) => Some(s3),
                                        None => {
                                            // Check second version
                                            match check_status(&swap_www(&u2).await).await {
                                                // If success, return
                                                Some(s4) => Some(s4),
                                                // Otherwise, this is broken
                                                None => None
                                            }
                                        }
                                    }
                                }
                                None => None
                            }
                        }
                    }
                }
            }
        }
        None => {
            println!("Error parsing URL");
            None
        }
    }
}