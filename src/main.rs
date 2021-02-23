use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    url: String,
}

#[tokio::main]
async fn fetch_page(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;

    println!("{}", response);
    Ok(())
}

fn main() {
    let opts = Opts::from_args();
    let _ = fetch_page(&opts.url);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use mockito;

    #[test]
    fn test_get() {
        let url = format!("{}/ip", &mockito::server_url());
        let mock = mock("GET", "/ip")
            .with_body("yep")
            .create();

        fetch_page(&url).expect("Failed GET");

        mock.assert();
    }
}

