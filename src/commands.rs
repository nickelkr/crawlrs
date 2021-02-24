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

    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let page = self.fetch_page(&self.base_url).expect("GET Failed");

        return Ok(page);
    }

    #[tokio::main]
    pub async fn fetch_page(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = reqwest::get(url)
            .await?
            .text()
            .await?;

        return Ok(response);
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
}
