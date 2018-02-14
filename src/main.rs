extern crate termion;

use std::{i8};
use std::io::{stdin, stdout, Write};
use termion::{clear, color};

fn main() {
    // Clear the terminal!
    println!("{}", clear::All);

    // Print my shit.
    println!("{yellow}AM Shell v0.1.0", yellow=color::Fg(color::Yellow));
    println!("Written by Daniel Owen van Dommelen\n");

    // Main loop.
    loop {
        print!(">");

        let _       = stdout().flush();
        let mut cmd = String::new();
        let input   = stdin().read_line(&mut cmd);
    }
}
