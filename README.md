<p align="center">
<img src="https://media.githubusercontent.com/media/rustic-rs/docs/main/assets/readme_header_scheduler.png" height="400" />
</p>
<p align="center"><b>centrally schedule rustic backups</b></p>

<!-- <p align="center">
<a href="https://crates.io/crates/rustic-rs"><img src="https://img.shields.io/crates/v/rustic-rs.svg" /></a>
<a href="https://docs.rs/rustic-rs/"><img src="https://img.shields.io/docsrs/rustic-rs?style=flat&amp;labelColor=1c1d42&amp;color=4f396a&amp;logo=Rust&amp;logoColor=white" /></a>
<a href="https://raw.githubusercontent.com/rustic-rs/rustic/main/"><img src="https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg" /></a>
<a href="https://crates.io/crates/rustic-rs"><img src="https://img.shields.io/crates/d/rustic-rs.svg" /></a>
<p> -->

## About

rustic scheduler is a client/server application to schedule regular backups on
many clients to one identical repository controlled by a central scheduling
server.

It allows to define client groups which are all backed up the same way.

**Note that rustic scheduler is in an early development stage. It uses the yet
not published `rustic_core` from the rustic crate as a git dependency.**

## Contact

| Contact       | Where?                                                                                        |
| ------------- | --------------------------------------------------------------------------------------------- |
| Issue Tracker | [GitHub Issues](https://github.com/rustic-rs/rustic_scheduler/issues)                         |
| Discord       | [![Discord](https://dcbadge.vercel.app/api/server/WRUWENZnzQ)](https://discord.gg/WRUWENZnzQ) |
| Discussions   | [GitHub Discussions](https://github.com/rustic-rs/rustic/discussions)                         |

## Getting started

- Install Rust, e.g. using rustup.
- Clone this github repository and the rustic repository.
- Compile the server and client using `cargo build --release --bins`
- Copy the `server` binary to your backup schedule server and the `client`
  binary to all your clients (available under `/targets/release`).
- Create a config file `rustic_schedulder.toml` on your backup schedule server
  (example config is available in the `config/` dir)
- Run the `server` binary on your server in the dir containing the config.
- On each client, run `client <ADDR>`, where `<ADDR>` is the websocket address
  to connect, e.g. `client http://server.localdomain:3012/ws`.
- Backups on your clients are automatically started based on the configured
  schedule(s).

## Are binaries available?

Not yet.

## Contributions

Contributions in form of [issues][4] or PRs are very welcome.

## License

Licensed under either of:

- [Apache License, Version 2.0](./LICENSE-APACHE)
- [MIT license](./LICENSE-MIT)

at your option.

[4]: https://github.com/rustic-rs/rustic_scheduler/issues/new/choose
