//! Utility functions

use regex::Regex;

use url::{Url, ParseError};
use reqwest::Client;

/// This function parses an incomplete URL
/// and returns the full URL.
///
/// ## Usage:
///
/// ```
/// use clean_url::utils::parse_url;
///
/// assert_eq!(Some(String::from("http://example.com/")), parse_url(String::from("example.com"), false));
/// assert_eq!(Some(String::from("https://example.com/")), parse_url(String::from("example.com"), true));
/// assert_eq!(Some(String::from("http://www.example.com/")), parse_url(String::from("www.example.com"), false));
/// assert_eq!(Some(String::from("https://www.example.com/")), parse_url(String::from("www.example.com"), true));
/// ```
pub fn parse_url(url: String, is_secure: bool) -> Option<String> {
    match Url::parse(url.as_str()) {
        Ok(u) => {
            println!("{:?}", u);
            Some(u.into_string())
        }
        Err(e) => {
            println!("{:?}", e);
            match e {
                ParseError::RelativeUrlWithoutBase => {
                    if is_secure {
                        parse_url(format!("https://{}", url), is_secure)
                    } else {
                        parse_url(format!("http://{}", url), is_secure)
                    }
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
/// use clean_url::utils::check_status;
/// use tokio_test::block_on;
///
/// assert_eq!(block_on(check_status("https://httpbin.org/status/200")), Some(String::from("https://httpbin.org/status/200")));
/// assert_eq!(block_on(check_status("http://www.bertsmithco.com")), Some(String::from("https://bertsmithco.com/")));
/// assert_eq!(block_on(check_status("https://www.bertsmithco.com")), Some(String::from("https://bertsmithco.com/")));
/// assert_eq!(block_on(check_status("https://www.jaredforthmusic.com")), Some(String::from("https://jaredforthmusic.com/")));
/// assert_eq!(block_on(check_status("http://www.jaredforthmusic.com")), Some(String::from("https://jaredforthmusic.com/")));
/// ```
pub async fn check_status(url: &str) -> Option<String> {
    let client = Client::new();
    match client.get(url).send().await {
        Ok(r) => {
            println!("{:?}", r.headers().contains_key("server"));
            let code = r.status();
            let resp_url = r.url();
            println!("{:?} - {:?}", code, resp_url);
            match r.headers().get("server") {
                Some(server) => {
                    if server.to_str().unwrap() == "Squarespace" {
                        // There is no hope, just return the URL
                        Some(resp_url.to_string())
                    }
                }
                None => None
            }
            if code.is_success() {
                Some(resp_url.to_string())
            } else {
                None
            }
        }
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}

lazy_static! {
    static ref WWW_RE: Regex = Regex::new(r"www\.").unwrap();
    static ref HTTP_RE: Regex = Regex::new(r"(?P<http>https?://)").unwrap();
}

/// Removes www if a URL has it, and
/// adds www if a URL does not
///
/// ## Usage:
///
/// ```
/// use clean_url::utils::swap_www;
///use tokio_test::block_on;
///
/// assert_eq!(String::from("http://www.example.com"), block_on(swap_www("http://example.com")));
/// assert_eq!(String::from("http://example.com"), block_on(swap_www("http://www.example.com")));
/// ```
pub async fn swap_www(url: &str) -> String {
    if has_www(url).await {
        remove_www(url).await
    } else {
        add_www(url).await
    }
}

/// Checks if a URL uses www
///
/// ## Usage:
///
/// ```
/// use clean_url::utils::has_www;
/// use tokio_test::block_on;
///
/// assert_eq!(true, block_on(has_www("http://www.example.com")));
/// assert_eq!(false, block_on(has_www("http://example.com")));
///
/// //assert_eq!(String::from("http://www.example.com"), swap_www("http://example.com"));
/// //assert_eq!(String::from("http://example.com"), swap_www("http://www.example.com"));
/// ```
pub async fn has_www(url: &str) -> bool {
    WWW_RE.is_match(url)
}

/// Removes www from a URL
///
/// ## Usage:
/// ```
/// use clean_url::utils::remove_www;
/// use tokio_test::block_on;
///
/// assert_eq!(String::from("http://example.com"), block_on(remove_www("http://www.example.com")));
/// ```
pub async fn remove_www(url: &str) -> String {
    WWW_RE.replace_all(url, "").to_string()
}

/// Adds www to a URL
///
/// ## Usage:
/// ```
/// use clean_url::utils::add_www;
/// use tokio_test::block_on;
///
/// assert_eq!(String::from("http://www.example.com"), block_on(add_www("http://example.com")));
/// ```
pub async fn add_www(url: &str) -> String {
    if !has_www(url).await {
        HTTP_RE.replace_all(url, "$http-www.").to_string().replace("-", "")
    } else {
        println!("Already has www");
        String::from(url)
    }
}