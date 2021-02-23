use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let opts = Opts::from_args();
    let response = reqwest::get(&opts.url)
        .await?
        .text()
        .await?;
    println!("{}", response);
    Ok(())
}
