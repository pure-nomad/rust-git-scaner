use clap::Parser;

use std::{
    fs,
    io::Result
};

use tokio::{
    sync::mpsc::unbounded_channel,
    task
};

mod scanner;
pub use crate::scanner::scanner as scan;
mod utils;
pub use crate::utils::utils as util;

/// Program that detects an exposed .git directory on websites. Made for ethical use only.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File containing Urls
    #[arg(short)]

    list: String,

    /// Number of Threads to Use
    #[arg(short, default_value_t = 50)]

    threads: i32,
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut url_vec: Vec<String> = Vec::new();
    let bind = fs::read_to_string(args.list)?;
    let file_iter = bind.lines();

    for line in file_iter {
        url_vec.push(line.to_string());
    }

    if url_vec.len() <= 0 {
        eprintln!("Please put some urls in your file friend");
    }


    if args.threads <= 0 {
        eprintln!("Stop fucking around");
    }

    let (sender, mut receiver) = unbounded_channel();

    let scanner = task::spawn(async move {
        let res = util::gen_ua().await.expect("error getting result from ua gen");
        println!("Random User Agent: {}",String::from(res.trim()));
        scan::git_scan(url_vec,args.threads,sender).await.expect("error scanning");
    });

    let result_handler = tokio::spawn(util::handle_result(receiver));

    let _ = tokio::join!(scanner, result_handler);

    Ok(())

    // if let Err(e) = git_scan(args., threads).await {
    //     eprintln!("Error scanning URLs: {}", e);
    // }
}
