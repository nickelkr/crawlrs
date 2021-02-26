use structopt::StructOpt;

mod commands;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    url: String,

    #[structopt(short, long, default_value = "1000")]
    timeout: u64,
}

fn main() {
    let opts = Opts::from_args();
    commands::Crawl::new(opts.url, opts.timeout).execute()
}
