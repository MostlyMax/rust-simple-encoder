use clap::Parser;
use log;

mod encode;
use crate::encode::encode_str;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// list of files to parse
    #[arg(required = true)]
    file: Vec<String>,

    /// number of threads to create
    #[arg(short, long, default_value_t = 1)]
    jobs: u8,
}


fn main() {
    env_logger::init();
    let args = Args::parse();

    log::debug!("files: {:?} - jobs: {:?}", args.file, args.jobs);
    encode_str("aaa");
}
