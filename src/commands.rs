use threadpool::ThreadPool;

use std::collections::BTreeSet;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::Duration;

mod helpers;

pub struct Crawl {
    base_url: String,
    timeout: u64,
    visited: BTreeSet<String>,
}

impl Crawl {
    pub fn new(base_url: String, timeout: u64) -> Self {
        let visited = BTreeSet::new();

        Crawl { base_url, timeout, visited }
    }

    pub fn execute(&mut self) {
        let (tx, rx) = mpsc::channel();
        let pool = ThreadPool::new(5);

        tx.send(self.base_url.to_string())
            .expect("Failed to send msg");
        loop {
            let link = match rx.recv_timeout(Duration::from_millis(self.timeout)) {
                Ok(link) => link,
                Err(RecvTimeoutError::Timeout) => {
                    break;
                }
                Err(RecvTimeoutError::Disconnected) => {
                    println!("Disconnected");
                    break;
                }
            };

            if self.has_visited(link.to_string()) { continue; }

            let tx = tx.clone();

            pool.execute(move || {
                let page = helpers::fetch_page(&link).expect("GET Failed");
                let extracted_links = helpers::extract_links(page).expect("EXTRACT failed");

                helpers::print_links(&link, &extracted_links);

                for link in extracted_links {
                    tx.send(link).expect("Failed to send msg");
                }
            });
        }

        pool.join();
        println!("Finished");
    }

    fn has_visited(&mut self, url: String) -> bool {
        if self.visited.contains(&url) {
            return true;
        }
        self.visited.insert(url);

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;
    use mockito::mock;

    #[test]
    fn test_execute() {
        let host = &mockito::server_url();
        let body = format!(
            "<html>
        <body>
            <a href=\"{}/two\">TWO</a>
        </body>
        </html>",
            host
        );
        let mock_index = mock("GET", "/").with_body(body).create();
        let mock_two = mock("GET", "/two").with_body("Ok").create();
        let mut crawl = Crawl::new(format!("{}/", host), 1000);

        crawl.execute();
        mock_index.assert();
        mock_two.assert();
    }

    #[test]
    fn test_has_visited() {
        let mut crawl = Crawl::new("http://example.com".to_string(), 100);

        assert_eq!(crawl.has_visited("example.com".to_string()), false);
        assert_eq!(crawl.has_visited("example.com".to_string()), true);
    }
}
