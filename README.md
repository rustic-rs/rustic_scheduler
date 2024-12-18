<p align="center">
<img src="https://raw.githubusercontent.com/rustic-rs/assets/main/logos/readme_header_scheduler.png" height="400" />
</p>
<p align="center"><b>centrally schedule rustic backups</b></p>
<p align="center">
<a href="https://crates.io/crates/rustic_scheduler"><img src="https://img.shields.io/crates/v/rustic_scheduler.svg" /></a>
<a href="https://docs.rs/rustic_scheduler/"><img src="https://img.shields.io/docsrs/rustic_scheduler?style=flat&amp;labelColor=1c1d42&amp;color=4f396a&amp;logo=Rust&amp;logoColor=white" /></a>
<a href="https://github.com/rustic-rs/rustic_scheduler/?tab=readme-ov-file#license"><img src="https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg" /></a>
<a href="https://crates.io/crates/rustic_scheduler"><img src="https://img.shields.io/crates/d/rustic_scheduler.svg" /></a>
<p>
<p align="center">
<a href="https://github.com/rustic-rs/rustic_scheduler/actions/workflows/nightly.yml"><img src="https://github.com/rustic-rs/rustic_scheduler/actions/workflows/nightly.yml/badge.svg" /></a>
</p>

## About

rustic scheduler is a client/server application to schedule regular backups on
many clients to one identical repository controlled by a central scheduling
server.

It allows to define client groups which are all backed up the same way.

**Note**: rustic scheduler is in an early development stage.

## Contact

| Contact       | Where?                                                                                        |
| ------------- | --------------------------------------------------------------------------------------------- |
| Issue Tracker | [GitHub Issues](https://github.com/rustic-rs/rustic_scheduler/issues)                         |
| Discord       | [![Discord](https://dcbadge.vercel.app/api/server/WRUWENZnzQ)](https://discord.gg/WRUWENZnzQ) |
| Discussions   | [GitHub Discussions](https://github.com/rustic-rs/rustic/discussions)                         |

### Installation

Copy the `rustic-scheduler` binary to your backup schedule server and to all
your clients. You can download the latest version from the
[releases page](https://github.com/rustic-rs/rustic_scheduler/releases)

## Getting started

- Create a config file `./config/rustic_scheduler.toml` on your backup schedule
  server (example config is available in the `config/` dir)

- Run the `rustic-scheduler` binary on your server in the dir containing the
  config.

- On each client, run `rustic-scheduler client --url <ADDR>`, where `<ADDR>` is
  the websocket address to connect, e.g.
  `rustic-scheduler client --url ws://server.localdomain:3012/ws`.

- Backups on your clients are automatically started based on the configured
  schedule(s).

- Statistics for a specific clients are available under `/client/%client`, e.g.
  `http://server.localdomain:3012/client/my_server1`.

## Contributing

Tried rustic-scheduler and not satisfied? Don't just walk away! You can help:

- You can report issues or suggest new features on our
  [Discord server](https://discord.gg/WRUWENZnzQ) or using
  [Github Issues](https://github.com/rustic-rs/rustic_scheduler/issues/new/choose)!

Do you know how to code or got an idea for an improvement? Don't keep it to
yourself!

- Contribute fixes or new features via a pull requests!

Please make sure, that you read the
[contribution guide](https://rustic.cli.rs/docs/contributing-to-rustic.html).

## License

Licensed under either of:

- [Apache License, Version 2.0](./LICENSE-APACHE)
- [MIT license](./LICENSE-MIT)

at your option.
