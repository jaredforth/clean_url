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