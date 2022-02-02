use rustix::net::SocketAddrUnix;
use std::os::unix::net::UnixStream;

fn env_key(key: &str) -> Option<String> {
    for (akey, value) in std::env::vars_os() {
        if akey == key {
            if let Ok(v) = value.into_string() { return Some(v) }
        }
    }
    None
}

pub fn read_session_address() -> Result<String, Box<dyn std::error::Error>> {
    Ok(env_key("DBUS_SESSION_BUS_ADDRESS").ok_or_else(|| "Environment variable not found")?)
    // TODO: according to the D-Bus spec, there are more ways to find the address, such
    // as asking the X window system.
}

pub fn read_system_address() -> Result<String, Box<dyn std::error::Error>> {
    Ok(env_key("DBUS_SYSTEM_BUS_ADDRESS").unwrap_or_else(||
        "unix:path=/var/run/dbus/system_bus_socket".into()
    ))
}

pub fn read_starter_address() -> Result<String, Box<dyn std::error::Error>> {
    Ok(env_key("DBUS_SESSION_BUS_ADDRESS").ok_or_else(|| "Environment variable not found")?)
}

fn path_sockaddr_un(s: &str) -> Result<SocketAddrUnix, Box<dyn std::error::Error>> {
    SocketAddrUnix::new(s).map_err(|_err| "Address too long".into())
}

fn abstract_sockaddr_un(s: &str) -> Result<SocketAddrUnix, Box<dyn std::error::Error>> {
    SocketAddrUnix::new_abstract_name(s.as_bytes()).map_err(|_err| "Address too long".into())
}

pub fn address_to_sockaddr_un(s: &str) -> Result<SocketAddrUnix, Box<dyn std::error::Error>> {
    if !s.starts_with("unix:") { Err("Address is not a unix socket")? };
    for pair in s["unix:".len()..].split(',') {
        let mut kv = pair.splitn(2, "=");
        if let Some(key) = kv.next() {
            if let Some(value) = kv.next() {
                if key == "path" { return path_sockaddr_un(value); }
                if key == "abstract" { return abstract_sockaddr_un(value) }
            }
        }
    }
    Err(format!("unsupported address type: {}", s))?
}

pub fn connect_blocking(addr: &str) -> Result<UnixStream, Box<dyn std::error::Error>> {
    let sockaddr = address_to_sockaddr_un(addr)?;
    crate::sys::connect_blocking(&sockaddr)
}

#[test]
fn bus_exists() {
    let addr = read_session_address().unwrap();
    println!("Bus address is: {:?}", addr);
    if addr.starts_with("unix:path=") {
        let path = std::path::Path::new(&addr["unix:path=".len()..]);
        assert!(path.exists());
    }

    let addr = read_system_address().unwrap();
    if addr.starts_with("unix:path=") {
        let path = std::path::Path::new(&addr["unix:path=".len()..]);
        assert!(path.exists());
    }
}
