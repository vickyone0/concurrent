use std::time::{Duration, Instant};

use reqwest::StatusCode;


async fn heartbeat(mut num: u32) {
    loop {
        println!("beating... {}", num);
        tokio::time::sleep(Duration::from_millis(25)).await;

        num +=1;
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let start_time = Instant::now();
    //tokio::spawn(heartbeat(0));
    //let (status_1,status_2) = tokio::join!(get_status("https://google.com"),get_status("https://google.com"));

    tokio::select! (
        stat = get_status("https://google.com")=> println!("status : {:?}",stat),
        stat = get_status("https://google.com")=> println!("status : {:?}",stat),
    );
    // println!("status 1: {}",status_1.unwrap());
    // println!("status 2: {}",status_2.unwrap());


    println!("Overall execution time: {}ms",start_time.elapsed().as_millis());
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(5);
    tokio::spawn(send_messages(tx));
    while let Some(message) = rx.recv().await {
        println!("Received message: '{}'", message);
    }
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

async fn receive_messages(mut rx: tokio::sync::mpsc::Receiver<String>){
while let Some(message) = rx.recv().await {
    println!("Received message: '{}'", message);
}

}


async fn send_messages(tx: tokio::sync::mpsc::Sender<String>){
    let mut message_nr = 1;

    loop {
        tx.send(format!("hello from async [{message_nr}]")).await.unwrap();

        message_nr +=1;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
