pub mod utils {
    use rayon::str::ParallelString;
    use reqwest;
    use tokio;

    pub async fn handle_result(mut receiver: tokio::sync::mpsc::UnboundedReceiver<i32>) {
        while let Some(result) = receiver.recv().await {
            println!("Received result: {}", result);
        }
    }

    pub async fn gen_ua() -> Result<String,reqwest::Error> {
        let ua_body: String = reqwest::get("https://raw.githubusercontent.com/microlinkhq/top-user-agents/refs/heads/master/src/index.json")
        .await?
        .text()
        .await?;

        ua_body.par_split_terminator(|c| c == '[' || c == ']');
        println!("LINES\n{:#?}",ua_body.par_lines());


        Ok("good to go".to_string())
    }
}