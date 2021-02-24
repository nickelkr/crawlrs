use structopt::StructOpt;

mod commands;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    url: String,
}


fn main() {
    let opts = Opts::from_args();
    let links = commands::Crawl::new(opts.url.clone())
                  .execute()
                  .expect("GET Failed");

    for link in links {
        println!("{}", link);
    }
}
