#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

use std::error::Error;

use thirtyfour::prelude::*;

pub static WEBPAGE_URL: &str = "http://127.0.0.1:42069";
pub static CHROME_DRIVER_URL: &str = "http://127.0.0.1:9515";
pub static FIREFOX_DRIVER_URL: &str = "http://127.0.0.1:4444";
pub static OPERA_DRIVER_URL: &str = "http://127.0.0.1:4444";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let chrome =
        WebDriver::new(CHROME_DRIVER_URL, DesiredCapabilities::chrome())
            .await?;
    chrome.goto(WEBPAGE_URL).await?;
    chrome.quit().await?;

    let firefox =
        WebDriver::new(FIREFOX_DRIVER_URL, DesiredCapabilities::firefox())
            .await?;
    firefox.goto(WEBPAGE_URL).await?;
    firefox.quit().await?;
    Ok(())
}
