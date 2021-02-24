use regex::Regex;

pub struct Crawl {
    base_url: String
}

impl Crawl {
    pub fn new(base_url: String) -> Self {
        let crawl = Crawl {
            base_url,
        };

        return crawl
    }

    pub fn execute(&self) -> Result<Vec<String>, regex::Error> {
        let page = self.fetch_page(&self.base_url).expect("GET Failed");
        let extracted_links = self.extract_links(page);

        return extracted_links;
    }

    #[tokio::main]
    async fn fetch_page(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = reqwest::get(url)
            .await?
            .text()
            .await?;

        return Ok(response);
    }

    fn extract_links(&self, page: String) -> Result<Vec<String>, regex::Error> {
        let regex = Regex::new("<a href=\"(?P<url>http[s]?://\\S+?)\">")?;
        let mut links: Vec<String> = vec![];

        for caps in regex.captures_iter(&page) {
            links.push(caps["url"].to_string());
        }

        return Ok(links);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;
    use mockito::mock;

    #[test]
    fn test_fetch_page() {
        let url = format!("{}/", &mockito::server_url());
        let mock = mock("GET", "/")
            .with_body("body")
            .create();
        let _ = Crawl::new(url).execute();

        mock.assert();
    }

    #[test]
    fn extract_links() {
        let data = "<html>
        <body>
            <p>sometext</p>
            <a href=\"https://somelink.com/blog\">block</a>
            <p>other text</p>
            <a href=\"https://anotherlink.com/time\">time</a>
            <p>end</p>
        </body>
        </html>";

        let crawler = Crawl::new(String::from("url"));
        let links = crawler.extract_links(data.to_string()).expect("Extract failed");

        assert_eq!(links,
            vec![
                "https://somelink.com/blog",
                "https://anotherlink.com/time"
            ]
        );
    }
}
