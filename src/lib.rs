use std::net::Ipv4Addr;
use std::time::Duration;

#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use win::*;

#[cfg(not(windows))]
mod unix;
#[cfg(not(windows))]
pub use unix::*;

pub struct MulticastOptions {
    pub read_timeout: Duration,
    pub loopback: bool,
    pub buffer_size: usize,
    /// The address to bind the socket to.
    ///
    /// Usually this will be Ipv4Addr::UNSPECIFIED, in order to listen for packets on all
    /// interfaces.
    pub bind_address: Ipv4Addr,
}

impl Default for MulticastOptions {
    fn default() -> Self {
        MulticastOptions {
            read_timeout: Duration::from_secs(1),
            loopback: true,
            buffer_size: 512,
            bind_address: Ipv4Addr::UNSPECIFIED,
        }
    }
}
