# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

## [0.1.2](https://github.com/rustic-rs/rustic_scheduler/compare/v0.1.1...v0.1.2) - 2024-11-09

### Other

- dprint fmt
- *(release)* add cargo dist packaging

## [0.1.1](https://github.com/rustic-rs/rustic_scheduler/compare/v0.1.0...v0.1.1) - 2024-11-09

### Fixed

- *(docs)* update readme
- *(docs)* update readme
- *(deps)* build sha2-asm on windows-gnu
- *(deps)* build sha2-asm on windows-gnu
- *(deps)* explicitly pull in sha2 asm extensions for non-windows targets
- *(deps)* update rust crate chrono to v0.4.38 ([#46](https://github.com/rustic-rs/rustic_scheduler/pull/46))
- *(deps)* update rust crate axum to v0.7.6 ([#45](https://github.com/rustic-rs/rustic_scheduler/pull/45))
- *(deps)* update rust crate anyhow to v1.0.89 ([#44](https://github.com/rustic-rs/rustic_scheduler/pull/44))
- typo in readme ([#37](https://github.com/rustic-rs/rustic_scheduler/pull/37))
- *(deps)* update rust crate serde_derive to 1.0.194 ([#33](https://github.com/rustic-rs/rustic_scheduler/pull/33))
- *(deps)* update rust crate tungstenite to 0.21 ([#34](https://github.com/rustic-rs/rustic_scheduler/pull/34))
- *(deps)* update rust crate axum to 0.7.3 ([#29](https://github.com/rustic-rs/rustic_scheduler/pull/29))
- *(deps)* update rust crate url to 2.5 ([#28](https://github.com/rustic-rs/rustic_scheduler/pull/28))
- *(deps)* update rust crate serde_derive to 1.0.193 ([#27](https://github.com/rustic-rs/rustic_scheduler/pull/27))
- *(ci)* remove unmaintained `actions-rs` ci actions
- *(deps)* update rust crate toml to 0.8.8 ([#25](https://github.com/rustic-rs/rustic_scheduler/pull/25))
- *(deps)* update rust crate serde_derive to 1.0.192 ([#24](https://github.com/rustic-rs/rustic_scheduler/pull/24))
- *(deps)* update rust crate clap_derive to 4.4.7 ([#23](https://github.com/rustic-rs/rustic_scheduler/pull/23))
- Remove redundant clone()
- *(deps)* update rust crate chrono to 0.4.31 ([#16](https://github.com/rustic-rs/rustic_scheduler/pull/16))
- *(deps)* update rust crate toml to 0.8.0 ([#12](https://github.com/rustic-rs/rustic_scheduler/pull/12))
- cloning
- build and clippy lints

### Other

- don't build on netbsd again, still fails due to `cannot find -lexecinfo`
- update readme
- *(deps)* update deps ([#60](https://github.com/rustic-rs/rustic_scheduler/pull/60))
- try fix build problems ([#58](https://github.com/rustic-rs/rustic_scheduler/pull/58))
- *(deps)* update actions ([#59](https://github.com/rustic-rs/rustic_scheduler/pull/59))
- use runners according to available images and target triple ([#57](https://github.com/rustic-rs/rustic_scheduler/pull/57))
- update dprint config ([#56](https://github.com/rustic-rs/rustic_scheduler/pull/56))
- add triage label to new issues only if no label has been set when creating it ([#55](https://github.com/rustic-rs/rustic_scheduler/pull/55))
- *(deps)* lock file maintenance rust dependencies ([#54](https://github.com/rustic-rs/rustic_scheduler/pull/54))
- ignore CHANGELOG.md in dprint formatting
- Update renovate.json
- use release-plz
- *(deps)* upgrade deps
- *(deps)* lockfile maintenance
- update deny.toml
- update deny.toml
- Revert "chore(deps): lock file maintenance" ([#49](https://github.com/rustic-rs/rustic_scheduler/pull/49))
- *(deps)* lock file maintenance ([#48](https://github.com/rustic-rs/rustic_scheduler/pull/48))
- *(deps)* update embarkstudios/cargo-deny-action action to v2 ([#47](https://github.com/rustic-rs/rustic_scheduler/pull/47))
- *(deps)* update taiki-e/install-action digest to 18ab6bd ([#43](https://github.com/rustic-rs/rustic_scheduler/pull/43))
- *(deps)* update swatinem/rust-cache digest to 23bce25 ([#42](https://github.com/rustic-rs/rustic_scheduler/pull/42))
- *(deps)* update obi1kenobi/cargo-semver-checks-action digest to 7272cc2 ([#41](https://github.com/rustic-rs/rustic_scheduler/pull/41))
- *(deps)* update embarkstudios/cargo-deny-action digest to 3f4a782 ([#40](https://github.com/rustic-rs/rustic_scheduler/pull/40))
- *(deps)* update actions/download-artifact digest to fa0a91b ([#39](https://github.com/rustic-rs/rustic_scheduler/pull/39))
- *(deps)* update actions/checkout digest to 692973e ([#38](https://github.com/rustic-rs/rustic_scheduler/pull/38))
- *(deps)* lock file maintenance ([#35](https://github.com/rustic-rs/rustic_scheduler/pull/35))
- break old ci jobs when new commits are pushed so we don't fill up the queue
- add project-cache-key for better caching in ci
- *(deps)* update taiki-e/install-action digest to 56ab793 ([#32](https://github.com/rustic-rs/rustic_scheduler/pull/32))
- *(deps)* update taiki-e/install-action digest to a9ad291 ([#31](https://github.com/rustic-rs/rustic_scheduler/pull/31))
- *(deps)* update actions/download-artifact action to v4 ([#30](https://github.com/rustic-rs/rustic_scheduler/pull/30))
- dprint fmt
- automerge lockfile maintenance
- activate automerge for github action digest update
- activate automerge for github action digest update
- *(fmt)* upgrade dprint config
- *(deps)* update taiki-e/install-action digest to d211c4b ([#26](https://github.com/rustic-rs/rustic_scheduler/pull/26))
- netbsd nightly builds fail due to missing execinfo, so we don't build on it for now
- update rustsec/audit-check
- update taiki-e/install-action
- update dtolnay/rust-toolchain
- Run actions that need secrets.GITHUB_TOKEN only on rustic-rs org
- lockfile maintenance
- *(deps)* update embarkstudios/cargo-deny-action digest to 1e59595 ([#22](https://github.com/rustic-rs/rustic_scheduler/pull/22))
- *(deps)* update taiki-e/install-action digest to 4d85042 ([#21](https://github.com/rustic-rs/rustic_scheduler/pull/21))
- *(deps)* update actions/checkout digest to b4ffde6 ([#20](https://github.com/rustic-rs/rustic_scheduler/pull/20))
- add results to ci
- update dprint plugins
- compile dependencies with optimizations in dev mode
- Add client/ site for client statistics ([#19](https://github.com/rustic-rs/rustic_scheduler/pull/19))
- Update README.md
- *(readme)* remove note about rustic_core not being published
- add x86_64-pc-windows-gnu target
- *(deps)* upgrade deps
- dprint fmt
- update cross ci
- *(cargo)* remove special os-dependent linker/compiler settings
- *(manifest)* use rustic_core from crates.io
- sign binaries using rsign2 as well
- add contributing
- fix list indent
- rewrite contributing remark
- relink to new image location
- *(deps)* update dependencies
- *(deps)* update actions/checkout action to v4 ([#18](https://github.com/rustic-rs/rustic_scheduler/pull/18))
- *(deps)* pin dependencies ([#17](https://github.com/rustic-rs/rustic_scheduler/pull/17))
- fmt
- add cargo deny
- *(deps)* update taiki-e/install-action digest to de0d48b ([#15](https://github.com/rustic-rs/rustic_scheduler/pull/15))
- add merge queue checks
- *(deps)* update actions/checkout action to v4 ([#13](https://github.com/rustic-rs/rustic_scheduler/pull/13))
- *(deps)* update taiki-e/install-action action to v2 ([#14](https://github.com/rustic-rs/rustic_scheduler/pull/14))
- *(deps)* update swatinem/rust-cache digest to a95ba19 ([#11](https://github.com/rustic-rs/rustic_scheduler/pull/11))
- *(deps)* pin dependencies ([#10](https://github.com/rustic-rs/rustic_scheduler/pull/10))
- run workflow on renovate branches
- update changelog
- run release checks also on release subbranches
- add triaging of issues
- run git-cliff with latest tag during release
- add dev tooling
- add changelog generation
- fix woggly github action comparison
- use bash substring comparison to determine package name from branch
- set right package
- fix github refs
- decrease build times on windows
- fix workflow name for create-binary-artifact action, and check breaking changes package dependent
- *(changelog)* add generated changelog
- add release CD
- fix comment being wrongly attributed
- update ci to reflect changes and optimizations from rustic_server
- remove lint from ci workflow and keep it separate, replace underscore in workflow file
- add comment about shallow clone
- declutter and reorganize
- add signature and shallow clones to nightly
- update header
- add link to nightly downloads in documentation
- *(readme)* add link to binaries and badge
- nightly builds
- refactor to library and client + server binaries
- fmt
- fix binary builds
- *(readme)* rewrite comment for rustic_core, uses git dependency now
- *(deps)* use workspace dependency on git repo
- add rustic to artifact build
- pull in rustic manually for now
- *(deps)* update dependencies
- add licenses, fix manifest
- fmt
- add ci
- Update rustic_scheduler.toml
- Update rustic_scheduler.toml
- initial commit

### Bug Fixes

- Build and clippy lints
- Cloning

### Documentation

- Add licenses, fix manifest
- Rewrite comment for rustic_core, uses git dependency now
- Add link to binaries and badge
- Add link to nightly downloads in documentation
- Add generated changelog

### Miscellaneous Tasks

- Add ci
- Pull in rustic manually for now
- Add rustic to artifact build
- Fix binary builds
- Nightly builds
- Update header
- Add signature and shallow clones to nightly
- Declutter and reorganize
- Add comment about shallow clone
- Remove lint from ci workflow and keep it separate, replace underscore in
  workflow file
- Update ci to reflect changes and optimizations from rustic_server
- Fix comment being wrongly attributed
- Add release CD
- Fix workflow name for create-binary-artifact action, and check breaking
  changes package dependent
- Decrease build times on windows
- Fix github refs
- Set right package
- Use bash substring comparison to determine package name from branch
- Fix woggly github action comparison
- Add changelog generation
- Add dev tooling
- Run git-cliff with latest tag during release
- Add triaging of issues
- Run release checks also on release subbranches

### Refactor

- Refactor to library and client + server binaries

<!-- generated by git-cliff -->
