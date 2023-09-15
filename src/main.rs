use std::io::{self, Write};
use std::thread::spawn;
use tungstenite::accept;
use std::net::TcpListener;

fn main () {
    print!("[IN] Enter the port that you want to run webSocket from: ");
    io::stdout().flush();
    let mut port: String = String::new();
    match io::stdin().read_line(&mut port){
        Ok(_) => {
            port=port;
        },
        Err(_) =>{
            println!("[OUT] Error while processing your input, doing default = 4444");
            port = String::from("4444");
        }
    }

    println!("[OUT] Chosen to proceed with host: {}", format!("{}{}", "127.0.0.1:", port));
    run_server(port);
}

fn run_server(port: String) {
    let form_port = String::from(format!("{}:{}", String::from("127.0.0.1"),port.trim()));
    println!("[+] Started Host: {}{}{}","ws://", form_port,"\nListening.\n\n");
    let server = TcpListener::bind(form_port).unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();

                if msg.is_binary() || msg.is_text() {
                    websocket.send(msg).unwrap();
                }
            }
        });
    }
}