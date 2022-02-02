use rustix::net::{AddressFamily, Protocol, SocketType};
use std::os::unix::net::UnixStream;

pub fn getuid() -> u32 {
    rustix::process::getuid().as_raw()
}

pub fn connect_blocking(addr: &rustix::net::SocketAddrUnix) -> Result<UnixStream, Box<dyn std::error::Error>> {
    // We have to do this manually because rust std does not support abstract sockets.
    // https://github.com/rust-lang/rust/issues/42048

    let fd = rustix::net::socket(AddressFamily::UNIX, SocketType::STREAM, Protocol::default())
        .map_err(|err| format!("Unable to create unix socket: {:?}", err))?;

    rustix::net::connect_unix(&fd, addr).map_err(|err| format!("Unable to connect to unix socket: {:?}", err))?;

    let u = UnixStream::from(fd);
    Ok(u)
}
