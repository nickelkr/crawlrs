use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    url: String,
}

fn main() {
    let opts = Opts::from_args();
    println!("{}", opts.url);
}
