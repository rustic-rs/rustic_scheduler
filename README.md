# rustic scheduler - centrally schedule rustic backups

## Contact

| Contact       | Where?                                                                                 |
| ------------- | -------------------------------------------------------------------------------------- |
| Issue Tracker | [GitHub Issues](https://github.com/rustic-rs/rustic_scheduler/issues)                  |
| Discord       | [![](https://dcbadge.vercel.app/api/server/WRUWENZnzQ)](https://discord.gg/WRUWENZnzQ) |
| Discussions   | [GitHub Discussions](https://github.com/rustic-rs/rustic/discussions)                  |

## About

Rustic scheduler is a client/server application to schedule regular backups on
many clients to the identicall repository controlled by a central scheduling
server.

It allows to define client groups which are all backuped the same way.

**Note that rustic scheduler is in an early development stage. It uses the yet
not published `rustic_core` from the rustic crate which needs to be in the path
`../rustic/crates/rustic_core/` in order to compile the applications.**

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

## Contribution

Contributions in form of [issues][1] or PRs are very welcome.

## License

Licensed under either of:

- [Apache License, Version 2.0](./LICENSE-APACHE)
- [MIT license](./LICENSE-MIT)

at your option.

[4]: https://github.com/rustic-rs/rustic_scheduler/issues/new/choose
