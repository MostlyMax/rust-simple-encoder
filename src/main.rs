use clap::Parser;
use log;

mod encode;
use crate::encode::run_encoder;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// list of files to parse
    #[arg(required = true)]
    files: Vec<String>,

    /// number of threads to create
    #[arg(short, long, default_value_t = 1)]
    jobs: u8,
}


fn main() {
    env_logger::init();
    let args = Args::parse();

    log::debug!("files: {:?} - jobs: {:?}", args.files, args.jobs);
    run_encoder(args.files, args.jobs).unwrap();
}
