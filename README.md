# multicast-socket

Single-socket to network across multiple interfaces

<a href="https://github.com/bltavares/multicast-socket/actions?query=workflow%3AQuickstart+branch%3Amaster">
    <img src="https://img.shields.io/github/workflow/status/bltavares/multicast-socket/Quickstart/master?label=main%20ci" />
</a>
<a href="https://github.com/bltavares/multicast-socket/actions?query=workflow%3ACross-compile+branch%3Amaster">
    <img src="https://img.shields.io/github/workflow/status/bltavares/multicast-socket/Cross-compile/master?label=cross%20ci" />
</a>
<a href="https://crates.io/crates/multicast-socket">
    <img src="https://img.shields.io/crates/v/multicast-socket.svg" />
</a>
<a href="https://docs.rs/multicast-socket">
    <img src="https://docs.rs/multicast-socket/badge.svg" />
</a>

This crate provides an abstract struct that allows a single-socket to listen and respond to multicast packets on multiple interfaces.

By default, when binding a regular `std::net::UdpSocket` to the unspecified ip `0.0.0.0`, the socket will only send messages to what the OS considers the `deafult interface`.
This is less than ideal on computers with multiple interfaces (a.k.a. 'multihomed') like laptops, cellphones, desktops with wifi, VPNs, Windows with WSL, Mac with Adapters and various common computer cenarios nowadays.

Instead of creating multiples sockets for each interface, and providing a multi-socket writer, this crate uses a couple of C/C++ OS-specific syscalls and methods to provide a single-socket multi-interface multicast experience.

The create also provides a utility constructor which uses `get_if_addrs` syscalls to find all available interfaces to bind to it, providing an out-of-the-box multicast multihomed experience.

The crate is designed with IPv4 in mind because I didn't have a test project to validate what IPv6 needs, and this is why the structs are IPv4 specific. If there is a suggestion on how to test this for IPv6, it could be expanded later.

## Examples

```sh
cargo run --example mdns
```

## Usage

```toml
[dependencies]
multicast-socket = "0.2.1"
```

## Targets

Main tier:

- x86_64-unknown-linux-gnu
- x86_64-pc-windows-msvc
- x86_64-apple-darwin

Cross tier:

- armv7-unknown-linux-gnueabihf
- aarch64-linux-android
- mips-unknown-linux-musl
- x86_64-unknown-linux-musl
- aarch64-unknown-linux-gnu

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## References

- <https://github.com/alexcrichton/socket2-rs/issues/86>
- <https://stackoverflow.com/questions/32278414/c-listen-to-multicast-on-all-interfaces-respond-on-same-as-recieved>
- <https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms741687(v=vs.85)>
- <https://github.com/alexcrichton/socket2-rs/issues/29>
- <https://github.com/nix-rust/nix/pull/990>
- <https://github.com/bltavares/colmeia/issues/15>
