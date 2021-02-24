use structopt::StructOpt;

mod commands;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    url: String,
}


fn main() {
    let opts = Opts::from_args();
    commands::Crawl::new(opts.url)
              .execute()
}
