use anyhow::{Context, Result};
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

const MAX_CONCURRENT_REQUESTS: usize = 5;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(180); // 3 minutes
const WOWHEAD_PREFIX: &str = "https://www.wowhead.com/talent-calc/blizzard/";

/// HTTP client for fetching talent builds from Archon.gg
pub struct ArchonFetcher {
    client: Client,
    semaphore: Arc<Semaphore>,
}

impl Default for ArchonFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchonFetcher {
    /// Create a new fetcher with default settings
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .pool_max_idle_per_host(10)
            .user_agent("ArchonConfigUpdater/1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS)),
        }
    }

    /// Fetch a talent build from Archon.gg and extract the talent string
    /// Returns None if:
    /// - HTTP 500 (insufficient data)
    /// - No talent link found in response
    /// - Request fails
    pub async fn fetch_talent_build(&self, url: &str) -> Result<Option<String>> {
        // Acquire semaphore permit to limit concurrent requests
        let _permit = self
            .semaphore
            .acquire()
            .await
            .context("Failed to acquire semaphore permit")?;

        // Make HTTP request
        let response = match self.client.get(url).send().await {
            Ok(resp) => resp,
            Err(e) => {
                // Log error but don't fail - some builds may not exist
                eprintln!("Failed to fetch {}: {}", url, e);
                return Ok(None);
            }
        };

        // Handle HTTP 500 as "no data available" (expected for new/unpopular builds)
        if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
            return Ok(None);
        }

        // Check for other error status codes
        if !response.status().is_success() {
            eprintln!("HTTP {} for {}", response.status(), url);
            return Ok(None);
        }

        // Parse HTML response
        let html = response.text().await.context("Failed to read response body")?;
        let talent_string = self.extract_talent_string(&html)?;

        Ok(talent_string)
    }

    /// Extract talent string from HTML response
    /// Looks for: <a href="https://www.wowhead.com/talent-calc/blizzard/...">
    /// Returns the talent string after stripping the prefix
    fn extract_talent_string(&self, html: &str) -> Result<Option<String>> {
        let document = Html::parse_document(html);

        // Find all anchor tags with href containing wowhead talent calc
        let selector = Selector::parse("a[href*='wowhead.com/talent-calc/blizzard/']")
            .map_err(|e| anyhow::anyhow!("Invalid selector: {:?}", e))?;

        // Get the first matching link
        let element = match document.select(&selector).next() {
            Some(el) => el,
            None => return Ok(None), // No talent link found
        };

        // Extract href attribute
        let href = match element.value().attr("href") {
            Some(h) => h,
            None => return Ok(None),
        };

        // Strip the Wowhead prefix to get the talent string
        if let Some(talent_string) = href.strip_prefix(WOWHEAD_PREFIX) {
            Ok(Some(talent_string.to_string()))
        } else {
            // Link doesn't have expected format
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_talent_string_from_html() {
        let fetcher = ArchonFetcher::new();

        let html = r#"
            <html>
                <body>
                    <div>
                        <a href="https://www.wowhead.com/talent-calc/blizzard/mage/frost/DABCabc123XYZ">Frost Mage Build</a>
                    </div>
                </body>
            </html>
        "#;

        let result = fetcher.extract_talent_string(html).unwrap();
        assert_eq!(result, Some("mage/frost/DABCabc123XYZ".to_string()));
    }

    #[test]
    fn test_extract_talent_string_no_link() {
        let fetcher = ArchonFetcher::new();

        let html = r#"
            <html>
                <body>
                    <div>No talent links here</div>
                </body>
            </html>
        "#;

        let result = fetcher.extract_talent_string(html).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_talent_string_wrong_format() {
        let fetcher = ArchonFetcher::new();

        let html = r#"
            <html>
                <body>
                    <a href="https://www.example.com/some-other-link">Other Link</a>
                </body>
            </html>
        "#;

        let result = fetcher.extract_talent_string(html).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_talent_string_multiple_links() {
        let fetcher = ArchonFetcher::new();

        let html = r#"
            <html>
                <body>
                    <a href="https://www.wowhead.com/talent-calc/blizzard/warrior/arms/ABC123">First</a>
                    <a href="https://www.wowhead.com/talent-calc/blizzard/warrior/fury/XYZ789">Second</a>
                </body>
            </html>
        "#;

        // Should return the first matching link
        let result = fetcher.extract_talent_string(html).unwrap();
        assert_eq!(result, Some("warrior/arms/ABC123".to_string()));
    }
}
