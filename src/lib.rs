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
}

impl Default for MulticastOptions {
    fn default() -> Self {
        MulticastOptions {
            read_timeout: Duration::from_secs(1),
            loopback: true,
            buffer_size: 512,
        }
    }
}
