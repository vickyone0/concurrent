use std::time::Instant;

use reqwest::StatusCode;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let start_time = Instant::now();
    let status_1 = get_status("https://google.com").await?;
    println!("status 1: {}",status_1);

    let status_2 = get_status("https://google.com").await?;

    println!("status 2:{}",status_2);

    println!("Overall execution time: {}ms",start_time.elapsed().as_millis());
    Ok(())
}


async fn foo() -> u32{
    println!("foo");
    5
}

async fn get_status(url: &str) -> Result<StatusCode, Box<dyn std::error::Error>>{

    let start_time = Instant::now();

    let status_code = reqwest::get(url).await?.status();

    let duration = start_time.elapsed().as_millis();

    println!("Took {} ma to featch url '{}'.",duration, url);
    Ok(status_code)
}