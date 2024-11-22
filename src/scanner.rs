pub mod scanner {
    
    use rayon::{
        iter::{IntoParallelRefIterator, ParallelIterator},
        ThreadPoolBuilder
    };
    
    use reqwest::ClientBuilder;

    use std::io::Result;

    pub async fn git_scan(urls: Vec<String>, threads: i32, sender: tokio::sync::mpsc::UnboundedSender<i32>) -> Result<()> {

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
    
            let mut _url_count: Vec<i32> = Vec::new();
            let mut c = 0;

    
    
            for _ in urls.iter() {
    
                println!("url in file");
                // success functionality (not writing yet)
    
                /* c+=1;
                url_count.push(c);*/
            }
    
            _url_count.par_iter().for_each(|&res| {
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
}