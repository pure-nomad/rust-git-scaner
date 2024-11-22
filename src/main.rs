use clap::Parser;

use std::{
    fs,
    io::{Result}
};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    ThreadPoolBuilder
};
use tokio::{
    sync::mpsc::unbounded_channel,
    task
};

use reqwest::{
  ClientBuilder
};

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

async fn git_scan(urls: Vec<String>, threads: i32, sender: tokio::sync::mpsc::UnboundedSender<i32>) -> Result<()> {

    let thread_pool = ThreadPoolBuilder::new()
        .num_threads((threads + 1) as usize)
        .build()
        .expect("Failed to create thread pool.");

    // optimize the client bro
    let req_client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .user_agent("User Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.169 Safari/537.3 Brave/1.23.76")
        .build();

    thread_pool.install(|| {

        let mut url_count: Vec<i32> = Vec::new();
        let mut c = 0;

        for _ in urls.iter() {



            // success functionality (not writing yet)

            /* c+=1;
            url_count.push(c);*/
        }

        url_count.par_iter().for_each(|&res| {
            if sender.send(res).is_err() {
                println!("Receiver dropped. Scanner stopped.");
            }
        });
    });


    // for url in urls.iter() {
    //     println!("URL {}",url);
    // }
    //
    // for i in 1..threads+1 {
    //     println!("thread {}",i)
    // }


    Ok(())
}

async fn handle_result(mut receiver: tokio::sync::mpsc::UnboundedReceiver<i32>) {
    while let Some(result) = receiver.recv().await {
        println!("Received result: {}", result);
    }
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
        panic!("Please put some urls in your file friend");
    }

    let (sender, mut receiver) = unbounded_channel();

    let scanner = task::spawn(async move {
        git_scan(url_vec,args.threads,sender).await.expect("error scanning");
    });

    let result_handler = tokio::spawn(handle_result(receiver));

    let _ = tokio::join!(scanner, result_handler);

    Ok(())

    // if let Err(e) = git_scan(args., threads).await {
    //     eprintln!("Error scanning URLs: {}", e);
    // }
}
