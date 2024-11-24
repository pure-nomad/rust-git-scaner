pub mod utils {
    use std::sync::{Arc, Mutex};

    use rayon::{iter::{IntoParallelIterator, ParallelIterator}, str::ParallelString};
    use reqwest;
    use tokio;
    use rand::Rng;

    pub async fn handle_result(mut receiver: tokio::sync::mpsc::UnboundedReceiver<i32>) {
        while let Some(result) = receiver.recv().await {
            println!("Received result: {}", result);
        }
    }

    pub async fn gen_ua() -> Result<String,reqwest::Error> {

        let mut ua_body: String = reqwest::get("https://raw.githubusercontent.com/microlinkhq/top-user-agents/refs/heads/master/src/index.json")
        .await?
        .text()
        .await?;

        // remove brackets
        ua_body = ua_body.replace(&['[',']'],"");
        let ua_vec = Arc::new(Mutex::new(Vec::new()));
        ua_body
        .par_lines()
        .into_par_iter()
        .for_each(|line|{
            // trim of new lines, replace commas and double quotes.
            ua_vec.lock().unwrap().push(line.trim().to_string().replace(&[',','"'],""));
        });

        let mut rng = rand::thread_rng();
        let ua_len: usize = ua_vec.lock().unwrap().len();
        let num: u32 = rng.gen_range(0..ua_len as u32);
        let string = format!("{:?}",ua_vec.lock().unwrap().get(num as usize));
        
        Ok(string)

        // lord please save me wtf is going on
    }
}