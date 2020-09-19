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


