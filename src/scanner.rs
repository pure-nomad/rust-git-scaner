pub mod scanner {
    
    use rayon::{
        iter::{IntoParallelRefIterator, ParallelIterator},
        ThreadPoolBuilder
    };
    
    use reqwest::{Client, ClientBuilder, Request, RequestBuilder};

    use std::io::Result;

    pub async fn git_scan(urls: Vec<String>, threads: i32, sender: tokio::sync::mpsc::UnboundedSender<i32>, ua: String) -> Result<()> {

        let thread_pool = ThreadPoolBuilder::new()
            .num_threads((threads + 1) as usize)
            .build()
            .expect("Failed to create thread pool.");
    
        // optimize the client bro
        let req_client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .user_agent(ua)
            .redirect(reqwest::redirect::Policy::limited(3))
            .connect_timeout(std::time::Duration::from_secs(8))
            .read_timeout(std::time::Duration::from_secs(8))
            .timeout(std::time::Duration::from_secs(8))
            .build()
            .expect("couldn't build http client");

    
        thread_pool.install(|| {
    
            let mut _url_count: Vec<i32> = Vec::new();
            let  _c = 0;

    
    
            for uri in urls.iter() {
                
                let hostname = reqwest::Url::parse(format!("{}/.git/HEAD",uri).as_str());
                let request = Request::new(reqwest::Method::GET, hostname.expect("error building request"));
                req_client.execute(request);


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