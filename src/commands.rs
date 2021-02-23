
#[tokio::main]
pub async fn fetch_page(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;

    return Ok(response);
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
        let _ = fetch_page(&url);

        mock.assert();
    }
}
