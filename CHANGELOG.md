## 2021-05-23, Version v0.2.2
### Commits
- [[`2a8c9309e9`](https://github.com/bltavares/multicast-socket/commit/2a8c9309e924513b0c6f44ade2d03382ed89ff6f)] (cargo-release) version 0.2.2 (Bruno Tavares)
- [[`788fce1b2a`](https://github.com/bltavares/multicast-socket/commit/788fce1b2a84f4ef0e9f88929a5056e4402c5775)] Merge pull request #12 from bltavares/clone-messages (Bruno Tavares)
- [[`e10da8fc14`](https://github.com/bltavares/multicast-socket/commit/e10da8fc14b43904b62b3fa6ad42a4f773bef574)] Make Messages clone (Bruno Tavares)

### Stats
```diff
 Cargo.toml  | 2 +-
 src/unix.rs | 4 ++--
 src/win.rs  | 4 ++--
 3 files changed, 5 insertions(+), 5 deletions(-)
```


## 2020-09-19, Version v0.2.0
### Commits
- [[`d807ea1941`](https://github.com/bltavares/multicast-socket/commit/d807ea19418005c7410d9c3eee426d284b72ee79)] (cargo-release) version 0.2.0 (Bruno Tavares)
- [[`5e5cbe6573`](https://github.com/bltavares/multicast-socket/commit/5e5cbe65733af03f0c45226c3de1046d64982eee)] Create a CHANGELOG.md file (Bruno Tavares)
- [[`297e99f96e`](https://github.com/bltavares/multicast-socket/commit/297e99f96eea320ae6ebfd9bf36cda53e323e083)] Merge pull request #10 from eskimor/rk-make-bind-address-configurable (Bruno Tavares)
- [[`f810f693b2`](https://github.com/bltavares/multicast-socket/commit/f810f693b2030a19565fe104491c43472dd21e81)] Merge pull request #11 from bltavares/wide-fix-due-to-new-behaviours (Bruno Tavares)
- [[`e5d43a8a4b`](https://github.com/bltavares/multicast-socket/commit/e5d43a8a4bef8db6fc7bf4071af66950974d016d)] Increase the buffer to fetch the interfaces on windows (Bruno Tavares)
- [[`e4bcd7b5d3`](https://github.com/bltavares/multicast-socket/commit/e4bcd7b5d36ea6e815c32942adf8bfc5d88e7965)] Fix when the same interface has multiple ips (Bruno Tavares)
- [[`9d41c523e4`](https://github.com/bltavares/multicast-socket/commit/9d41c523e4c29227d7725de5d273c1f4525d2ae6)] Ensure that the test of the socket options build is being validate (Bruno Tavares)
- [[`5ec7087244`](https://github.com/bltavares/multicast-socket/commit/5ec7087244d0811cb059f276a85b3e3afeb6ab72)] Make bind address configurable (Robert Klotzner)
- [[`42893cd05d`](https://github.com/bltavares/multicast-socket/commit/42893cd05df51262d02d585acb21c8a9e90e1c08)] Merge pull request #9 from eskimor/rk-document-IpvPacketInfo (Bruno Tavares)
- [[`d4e581f966`](https://github.com/bltavares/multicast-socket/commit/d4e581f966c2419cf179742c08181228b3eb7d90)] Document usage of IP_PKTINFO (Robert Klotzner)
- [[`44a3d55e5c`](https://github.com/bltavares/multicast-socket/commit/44a3d55e5c25d3bb72150a3de42ff6a0af4da85b)] Update README.md (Bruno Tavares)

### Stats
```diff
 CHANGELOG.md     | 37 +++++++++++++++++++++++++++++++++++++
 Cargo.toml       |  4 ++--
 README.md        |  2 --
 examples/mdns.rs |  6 ++++--
 src/lib.rs       |  7 +++++++
 src/unix.rs      | 32 +++++++++++++++++++-------------
 src/win.rs       | 17 +++++++++++------
 7 files changed, 80 insertions(+), 25 deletions(-)
```


## 2020-08-29, Version v0.1.0
### Commits
- [[`459a35963b`](https://github.com/bltavares/multicast-socket/commit/459a35963b1957d38da1ab946667de7b15e0890a)] Update the version to released crates (Bruno Tavares)
- [[`89065dd3c9`](https://github.com/bltavares/multicast-socket/commit/89065dd3c919c2d636d2348c9f84573e77127fc4)] Merge pull request #5 from bltavares/fix-windows (Bruno Tavares)
- [[`d6b49905e4`](https://github.com/bltavares/multicast-socket/commit/d6b49905e4cb9d265215e14a364df62fd235e43c)] Fix windows packet creation (Bruno Tavares)
- [[`4f1a75a4a4`](https://github.com/bltavares/multicast-socket/commit/4f1a75a4a40ad98d7a743e079349c7e6fceceb18)] Merge pull request #4 from bltavares/expose-build-options (Bruno Tavares)
- [[`663b65fb2f`](https://github.com/bltavares/multicast-socket/commit/663b65fb2f5a3690d8dea7681a1c0a0c5fed2f59)] Expose Build options publically (Bruno Tavares)
- [[`a6eb53660e`](https://github.com/bltavares/multicast-socket/commit/a6eb53660e4ce9bdaf46811799457adb06e0ae2b)] Merge pull request #3 from bltavares/windows (Bruno Tavares)
- [[`e7a446f9c3`](https://github.com/bltavares/multicast-socket/commit/e7a446f9c3121e9e2acf404121163674cdb3123f)] Use sendmsg on windows as well (Bruno Tavares)
- [[`2095ec75df`](https://github.com/bltavares/multicast-socket/commit/2095ec75df798255465d43855770632a4cdda5c5)] Merge pull request #2 from bltavares/mobile-compile (Bruno Tavares)
- [[`066a0401f8`](https://github.com/bltavares/multicast-socket/commit/066a0401f8542f3c58bd509fbeccac845b4cba91)] Add checks for ios as well (Bruno Tavares)
- [[`be0237480c`](https://github.com/bltavares/multicast-socket/commit/be0237480cf7a94397171965f9db7b72eb3632fe)] Make it work on mobile as well (Bruno Tavares)
- [[`0232abd48d`](https://github.com/bltavares/multicast-socket/commit/0232abd48df8b05e914cdcfd1df0c52fb76fcd59)] Merge pull request #1 from bltavares/macos (Bruno Tavares)
- [[`18a75ccf08`](https://github.com/bltavares/multicast-socket/commit/18a75ccf088951c3683a024963e944efcc8a36d8)] README (Bruno Tavares)
- [[`67ab3899f7`](https://github.com/bltavares/multicast-socket/commit/67ab3899f7928c5d78063b224d9aaa8bc4aa95c4)] Make it compile on MacOS, ARM(64), Android and MIPS (Bruno Tavares)
- [[`15b322c359`](https://github.com/bltavares/multicast-socket/commit/15b322c3592e9380da12604ebc0f4497d068e96b)] Trying to compile to macos using the interface index to set the interface on send (Bruno Tavares)
- [[`32e6ad8a6d`](https://github.com/bltavares/multicast-socket/commit/32e6ad8a6d33b5449c2f981f602ceb3e70c3acca)] Ingore vscode (Bruno Tavares)
- [[`42acbd6038`](https://github.com/bltavares/multicast-socket/commit/42acbd6038d7986ae9bdd20279555c047afaf079)] Remove clippy while warnings exists (Bruno Tavares)
- [[`90a75d025c`](https://github.com/bltavares/multicast-socket/commit/90a75d025c121e7017da78f1be83481b88143548)] Initial commit - crate launch (Bruno Tavares)

### Stats
```diff
 .github/workflows/cross_compile.yml |  52 ++++-
 .github/workflows/main.yml          |  82 +++++++-
 .gitignore                          |   3 +-
 Cargo.toml                          |  28 ++-
 LICENSE-APACHE                      | 201 +++++++++++++++++-
 LICENSE-MIT                         |  21 ++-
 README.md                           |  82 +++++++-
 examples/mdns.rs                    |  36 +++-
 src/lib.rs                          |  27 ++-
 src/unix.rs                         | 194 ++++++++++++++++-
 src/win.rs                          | 444 +++++++++++++++++++++++++++++++++++++-
 11 files changed, 1170 insertions(+)
```


