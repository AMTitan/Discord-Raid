use curl::easy::{Easy, List};
use text_io::read;
use std::io::Split;
use std::{thread, time};

fn main() {
    println!("What is your tokens? (you can paste it in the terminal and type stop when you are done)");
    let mut tokens = Vec::new();
    let mut stop = false;
    while !stop {
        let token:String = read!();
        if token.as_str().to_lowercase() == "stop" {
            stop = true;
        }
        else {
            let split:Vec<String> = token.trim().split("\n").map(String::from).collect();
            for i in split {
                tokens.push(i);
            }   
        }
    }
    println!("What is your invite code? (ex. SXJdxTxr)");
    let invite:String = read!();
    println!("What is your timeout between messages in ms? (ex. 250)");
    let timeout:String = read!();
    println!("What do you want your message to be? (ex. This Guild is Hacked)");
    let message:String = read!();
    println!("Where do you want this message sent, I need the channel id? (ex. 234523495734587)");
    let channel_id:String = read!();
    for token in tokens {
        join(invite.to_string(), token.clone());
        message_send(channel_id.to_string(), token.to_string(), message.to_string());
        thread::sleep(time::Duration::from_millis(timeout.to_string().parse::<u64>().unwrap()));
    }
}

fn message_send(channel:String, token:String, message:String) {
    let mut easy = Easy::new();
    easy.url(["https://discordapp.com/api/v6/channels/", channel.as_str(), "/messages"].join("").as_str()).unwrap();
    let _output = easy.custom_request("POST");
    let mut list = List::new();
    list.append(["authorization: ", token.as_str()].join("").as_str()).unwrap();
    list.append("Content-Type: application/json").unwrap();
    list.append(["application/json: {\r\n \"content\": \"", message.as_str(), "\"\r\n}"].join("").as_str()).unwrap();
    easy.http_headers(list).unwrap();

    let mut transfer = easy.transfer();
    transfer
        .write_function(|data| {
            Ok(data.len())
        })
        .unwrap();
    transfer.perform().unwrap();
    drop(transfer);
}

fn join(invite:String, token:String) {
    let mut easy = Easy::new();
    easy.url(["https://discordapp.com/api/v6/invite/", invite.as_str()].join("").as_str()).unwrap();
    let _output = easy.custom_request("POST");
    let mut list = List::new();
    list.append(["authorization: ", token.as_str()].join("").as_str()).unwrap();
    easy.http_headers(list).unwrap();

    let mut transfer = easy.transfer();
    transfer
        .write_function(|data| {
            Ok(data.len())
        })
        .unwrap();
    transfer.perform().unwrap();
    drop(transfer);
}
