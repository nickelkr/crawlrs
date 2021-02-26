use regex::Regex;
use threadpool::ThreadPool;

use std::collections::BTreeSet;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::Duration;

pub struct Crawl {
    base_url: String,
}

impl Crawl {
    pub fn new(base_url: String) -> Self {
        Crawl { base_url }
    }

    pub fn execute(&self) {
        let (tx, rx) = mpsc::channel();
        let mut visited = BTreeSet::new();
        let pool = ThreadPool::new(5);

        tx.send(self.base_url.to_string())
            .expect("Failed to send msg");
        loop {
            let link = match rx.recv_timeout(Duration::from_millis(1000)) {
                Ok(link) => link,
                Err(RecvTimeoutError::Timeout) => {
                    break;
                }
                Err(RecvTimeoutError::Disconnected) => {
                    println!("Disconnected");
                    break;
                }
            };

            if visited.contains(&link) {
                continue;
            }

            visited.insert(link.to_string());
            let tx = tx.clone();

            pool.execute(move || {
                let page = fetch_page(&link).expect("GET Failed");
                let extracted_links = extract_links(page).expect("EXTRACT failed");

                print_links(&link, &extracted_links);

                for link in extracted_links {
                    tx.send(link).expect("Failed to send msg");
                }
            });
        }

        pool.join();
        println!("Finished");
    }
}

#[tokio::main]
async fn fetch_page(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("DEBUG: fetch page {}", &url);
    let response = reqwest::get(url).await?.text().await?;

    Ok(response)
}

fn extract_links(page: String) -> Result<Vec<String>, regex::Error> {
    let regex = Regex::new("<a href=\"(?P<url>http[s]?://\\S+?)\">")?;
    let mut links: Vec<String> = vec![];

    for caps in regex.captures_iter(&page) {
        links.push(caps["url"].to_string());
    }

    Ok(links)
}

fn print_links(url: &str, links: &[String]) {
    let formatted = links.join("\n\t");
    println!("{}\n\t{}", url, formatted);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;
    use mockito::mock;

    #[test]
    fn test_fetch_page() {
        let url = format!("{}/", &mockito::server_url());
        let mock = mock("GET", "/").with_body("body").create();
        let _ = fetch_page(&url);

        mock.assert();
    }

    #[test]
    fn test_extract_links() {
        let data = "<html>
        <body>
            <p>sometext</p>
            <a href=\"https://somelink.com/blog\">block</a>
            <p>other text</p>
            <a href=\"https://anotherlink.com/time\">time</a>
            <p>end</p>
        </body>
        </html>";

        let links = extract_links(data.to_string()).expect("Extract failed");

        assert_eq!(
            links,
            vec!["https://somelink.com/blog", "https://anotherlink.com/time"]
        );
    }
}
