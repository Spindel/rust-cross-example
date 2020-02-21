use std::env::consts::{ARCH, OS};

extern crate uname;
use uname::uname;

extern crate dbus;
use dbus::{BusType, Connection};

const DBUS_TIMEOUT: i32 = 5000;
const HOSTNAMED_SERVICE: &str = "org.freedesktop.hostname1";
const HOSTNAMED_IFACE: &str = HOSTNAMED_SERVICE;
const HOSTNAMED_PATH: &str = "/org/freedesktop/hostname1";

fn dbus_hostname() -> Result<String, Box<dyn std::error::Error>> {
    let c = Connection::get_private(BusType::System)?;
    let p = c.with_path(HOSTNAMED_SERVICE, HOSTNAMED_PATH, DBUS_TIMEOUT);

    use dbus::stdintf::org_freedesktop_dbus::Properties;

    let hostname: String = p.get(HOSTNAMED_IFACE, "Hostname")?;
    Ok(hostname)
}

fn main() {
    println!("Hello World from {} on {}", OS, ARCH);
    let info = uname().unwrap();
    println!(
        "{} {} {} {} {}",
        info.sysname, info.nodename, info.release, info.version, info.machine
    );

    let hostname = dbus_hostname();
    match hostname {
        Ok(v) => println!("Dbus hostname: {}", v),
        Err(_e) => println!("DBus connection failed."),
    }
}
