extern crate termion;
extern crate hyper;

use std::io::{self, stdin, stdout, Read, Write};
use termion::{clear, color};
use hyper::Client;

fn main() {
    // Clear the terminal!
    println!("{}", clear::All);

    // Print my shit.
    println!("{yellow}AM Shell v0.1.0", yellow=color::Fg(color::Yellow));
    println!("Written by Daniel Owen van Dommelen\n");

    // Test out a quick GET request.
    let client = Client::new();
    let url = "http://httpbin.org/status/418";
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => panic!("Whoops."),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };
    println!("buf: {}", buf);

    // Main loop.
    loop {
        print!("{white}>", white=color::Fg(color::White));

        let _       = stdout().flush();
        let mut cmd = String::new();
        let input   = stdin().read_line(&mut cmd);
    }
}
