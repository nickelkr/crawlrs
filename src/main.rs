use structopt::StructOpt;

mod commands;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    url: String,
}


fn main() {
    let opts = Opts::from_args();
    let page = commands::Crawl::new(opts.url.clone())
                  .execute()
                  .expect("GET Failed");

    println!("{}", page);
}
