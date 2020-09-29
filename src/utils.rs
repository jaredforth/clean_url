//! Utility functions

use url::{Url, ParseError};
use reqwest::blocking::Client;

/// This function parses an incomplete URL
/// and returns the full URL.
///
/// ## Usage:
///
/// ```
/// use servstat::utils::parse_url;
///
/// assert_eq!(Some(String::from("http://example.com/")), parse_url(String::from("example.com")));
/// assert_eq!(Some(String::from("http://www.example.com/")), parse_url(String::from("www.example.com")));
/// ```
pub fn parse_url(url: String) -> Option<String> {
    match Url::parse(url.as_str()) {
        Ok(u) => {
            println!("{:?}", u);
            Some(u.into_string())
        }
        Err(e) => {
            println!("{:?}", e);
            match e {
                ParseError::RelativeUrlWithoutBase => {
                    parse_url(format!("http://{}", url))
                }
                _ => { None }
            }
        }
    }
}

/// This function checks the status code
/// of a URL and returns the version of the
/// URL that is a 200 series.
///
/// ## Usage:
///
/// ```
/// use servstat::utils::check_status;
///
/// assert_eq!(check_status(String::from("https://httpbin.org/status/200")), Some(String::from("https://httpbin.org/status/200")));
/// assert_eq!(check_status(String::from("http://www.bertsmithco.com")), Some(String::from("https://bertsmithco.com/")));
/// assert_eq!(check_status(String::from("https://www.bertsmithco.com")), Some(String::from("https://bertsmithco.com/")));
/// ```
pub fn check_status(url: String) -> Option<String> {
    let client = Client::new();
    match client.get(url.as_str()).send() {
        Ok(r) => {
            let code = r.status();
            let resp_url = r.url();
            println!("{:?} - {:?}", code, resp_url);
            if code.is_success() {
                Some(resp_url.to_string())
            } else {
                // TODO Does the URL use www? If so, remove it
                // TODO Does the URL not use www? If so, add it
                None
            }
        }
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}