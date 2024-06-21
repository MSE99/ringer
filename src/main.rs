mod config;

use std::sync::mpsc;

fn main() {
    let (sender, receiver) = mpsc::channel();

    ctrlc::set_handler(move || {
        match sender.send(true) {
            Ok(_) => (),
            Err(e) => println!("{}", e.to_string()),
        };
        ()
    })
    .expect("could not set interrupt signal handler");

    receiver.recv().unwrap();

    println!("{}", "Shutting down");
}
