use regex::Regex;

#[tokio::main]
pub async fn fetch_page(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;

    Ok(response)
}

pub fn extract_links(page: String) -> Result<Vec<String>, regex::Error> {
    let regex = Regex::new("<a href=\"(?P<url>http[s]?://\\S+?)\">")?;
    let mut links: Vec<String> = vec![];

    for caps in regex.captures_iter(&page) {
        links.push(caps["url"].to_string());
    }

    Ok(links)
}

pub fn print_links(url: &str, links: &[String]) {
    let formatted = links.join("\n\t");
    println!("{}\n\t{}", url, formatted);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{self, mock};

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
