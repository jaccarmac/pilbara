#![feature(conservative_impl_trait)]
#![feature(use_extern_macros)]

extern crate clap;

use clap::clap_app;

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
    move |s| {
        s.parse::<VT>()
            .or_else(|_| Err(format!("\"{}\" is not valid", s)))
            .and_then(|v| validator(v).map_err(|e| e.to_string()))
            .map_err(|e| format!("bad {}: {}", typedesc, e))
            .and(Ok(()))
    }
}

fn in_port_number_range(port_number: u32) -> Result<(), &'static str> {
    if port_number <= 65_536 {
        Ok(())
    } else {
        Err("out of range (0 - 65,536)")
    }
}
