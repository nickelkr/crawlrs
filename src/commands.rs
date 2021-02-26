use threadpool::ThreadPool;

use std::collections::BTreeSet;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::Duration;

mod helpers;

pub struct Crawl {
    base_url: String,
    timeout: u64,
}

impl Crawl {
    pub fn new(base_url: String, timeout: u64) -> Self {
        Crawl { base_url, timeout }
    }

    pub fn execute(&self) {
        let (tx, rx) = mpsc::channel();
        let mut visited = BTreeSet::new();
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

            if visited.contains(&link) {
                continue;
            }

            visited.insert(link.to_string());
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
        let crawl = Crawl::new(format!("{}/", host), 1000);

        crawl.execute();
        mock_index.assert();
        mock_two.assert();
    }
}
