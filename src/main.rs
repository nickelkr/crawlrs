use structopt::StructOpt;

mod commands;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    url: String,
}


fn main() {
    let opts = Opts::from_args();
    let page = commands::fetch_page(&opts.url).expect("GET FAILED");

    println!("{}", page);
}
