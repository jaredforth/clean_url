use url::{Url, ParseError};

/// Returns a URL
///
/// ## Usage:
///
/// ```
/// use servstat::return_url;
///
/// assert_eq!(Some(String::from("http://example.com/")), return_url(String::from("example.com")));
/// assert_eq!(Some(String::from("http://www.example.com/")), return_url(String::from("www.example.com")));
/// ```
pub fn return_url(url: String) -> Option<String> {
    parse_url(url)
}

fn parse_url(url: String) -> Option<String> {
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