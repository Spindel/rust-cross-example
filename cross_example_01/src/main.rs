use std::env::consts::{ARCH, OS};
extern crate uname;
use uname::uname;

fn main() {
    println!("Hello World from {} on {}", OS, ARCH);
    let info = uname().unwrap();

    println!(
        "{} {} {} {} {}",
        info.sysname, info.nodename, info.release, info.version, info.machine
    );
}
