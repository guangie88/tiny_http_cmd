extern crate hyper;
extern crate tiny_http;

use hyper::client::{Client, Response};
use std::time::Duration;
use std::io::Read;
use std::process::{Command, Output};
use std::thread;
use tiny_http::Server;

// client

fn read_body(rsp: &mut Response) -> String {
    let mut s = String::new();
    rsp.read_to_string(&mut s).unwrap();
    s
}

// server

fn exec_cmd(cmd: &str) -> Output {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", cmd]).output()
    } else {
        Command::new("sh").args(&["-c", cmd]).output()
    }.unwrap()
}

fn execute() -> String {
    const CMD: &str = "echo hello";
    format!("{:?}", exec_cmd(CMD))
}

fn main() {
    thread::spawn(|| {
        // client thread
        const WAIT_MS: u64 = 1000;
        const LOOP_COUNT: usize = 50;

        println!("Client sleeping for {} ms before starting...", WAIT_MS);
        thread::sleep(Duration::from_millis(WAIT_MS));

        let cmd_map = (0..LOOP_COUNT)
            .map(|index| {
                let client = Client::new();
                let mut rsp = client.get("http://localhost:8000/").send().unwrap();
                (index, read_body(&mut rsp))
            });

        for (index, body) in cmd_map {
            println!("#{:02}: {}", index, body);
        }

        println!("Client completed! Press CTRL-C to exit...");
    });

    // server start
    let server = Server::http("localhost:8000").unwrap();

    loop {
        let req = server.recv().unwrap();
        let _ = req.respond(tiny_http::Response::from_string(execute()));
    }
}