# instagram-scraper-rs

<p align="center">~ Scrapes an instagram user's photos and videos ~</p>
<p align="center">
  <a href="#get-started-">Get started</a>
  ·
  <a href="https://docs.rs/instagram-scraper-rs" target="_blank">Documentation</a>
</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.1.0 (10/09/2022)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/instagram-scraper-rs/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/instagram-scraper-rs.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/instagram-scraper-rs"
    ><img
      src="https://img.shields.io/crates/d/instagram-scraper-rs.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/instagram-scraper-rs"
    ><img
      src="https://img.shields.io/crates/v/instagram-scraper-rs.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/instagram-scraper-rs/actions"
    ><img
      src="https://github.com/veeso/instagram-scraper-rs/workflows/Build/badge.svg"
      alt="Build CI"
  /></a>
  <a href="https://coveralls.io/github/veeso/instagram-scraper-rs"
    ><img
      src="https://coveralls.io/repos/github/veeso/instagram-scraper-rs/badge.svg"
      alt="Coveralls"
  /></a>
  <a href="https://docs.rs/instagram-scraper-rs"
    ><img
      src="https://docs.rs/instagram-scraper-rs/badge.svg"
      alt="Docs"
  /></a>
</p>

---

- [instagram-scraper-rs](#instagram-scraper-rs)
  - [About instagram-scraper-rs 📷](#about-instagram-scraper-rs-)
  - [Features 🎁](#features-)
  - [Get started 🏁](#get-started-)
    - [Add instagram-scraper-rs to your Cargo.toml 🦀](#add-instagram-scraper-rs-to-your-cargotoml-)
    - [Examples 🔍](#examples-)
  - [Documentation 📚](#documentation-)
  - [Support the developer ☕](#support-the-developer-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

---

## About instagram-scraper-rs 📷

instagram-scraper-rs is a Rust library that scrapes and downloads an instagram user's photos and videos. Use responsibly.
It is basically a 1:1 copy of the Python [Instagram-scraper](https://github.com/arc298/instagram-scraper) cli application.

## Features 🎁

- Query profile information
- Collect the user's profile picture
- Collect users' posts
- Collect users' stories
- Totally async

---

## Get started 🏁

### Add instagram-scraper-rs to your Cargo.toml 🦀

```toml
instagram-scraper-rs = "^0.1.0"
```

Supported features are:

- `no-log`: disable logging
- `native-tls` (*default*): use native-tls for reqwest
- `rustls`: use rustls for reqwest (you must disable default features)

### Examples 🔍

You can check the example to scrape an instagram account running the example, which is located at `examples/scraper.rs`:

```sh
cargo run --example scraper
```

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/instagram-scraper-rs>

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve instagram-scraper-rs, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View instagram-scraper-rs's changelog [HERE](CHANGELOG.md)

---

## License 📃

instagram-scraper-rs is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
