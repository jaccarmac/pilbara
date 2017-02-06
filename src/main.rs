//! A configurable MUD client using TCOD for rendering. Designed around support
//! for Discworld MUD, though that won't be the only target by any means.

extern crate argparse;

use argparse::{ArgumentParser, Store};

fn main() {
    let mut host = "".to_string();
    let mut port: u16 = 0;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Configurable MUD client in TCOD.");
        ap.refer(&mut host)
            .add_argument("host",
                          Store,
                          "hostname of MUD server: discworld.starturtle.net")
            .required();
        ap.refer(&mut port)
            .add_argument("port", Store, "port to connect to: 4242")
            .required();
        ap.parse_args_or_exit();
    }
    println!("Hello, world!");
}
