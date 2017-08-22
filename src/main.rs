//! A configurable MUD client using TCOD for rendering. Designed around support
//! for Discworld MUD, though that won't be the only target by any means.
#![feature(conservative_impl_trait)]
#![feature(use_extern_macros)]

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap::clap_app!((clap::crate_name!()) =>
        (version: clap::crate_version!())
        (author: clap::crate_authors!())
        (@arg host: * "MUD server to connect to:\tdiscworld.starturtle.net")
        (@arg port:
            *
            {can_convert_to_valid("port number", in_port_number_range)}
            "port on the server to connect to:\t4242")
        ).get_matches();
    println!(
        "Connecting to {}:{}",
        matches.value_of("host").unwrap(),
        matches.value_of("port").unwrap()
    );
}

fn can_convert_to_valid<
    D: std::fmt::Display,
    V: Fn(VT) -> Result<VO, VE>,
    VT: std::str::FromStr,
    VO,
    VE: std::fmt::Display,
>(
    typedesc: D,
    validator: V,
) -> impl Fn(String) -> Result<(), String> {
    move |s| match match s.parse::<VT>() {
        Ok(value) => {
            match validator(value) {
                Ok(_) => Ok(()),
                Err(reason) => Err(reason.to_string()),
            }
        }
        Err(_) => Err(format!("\"{}\" is not valid", s)),
    } {
        ok @ Ok(_) => ok,
        Err(reason) => Err(format!("bad {}: {}", typedesc, reason)),
    }
}

fn in_port_number_range(port_number: u32) -> Result<(), &'static str> {
    match port_number <= 65536 {
        true => Ok(()),
        false => Err("out of range (0 - 65536)"),
    }
}
