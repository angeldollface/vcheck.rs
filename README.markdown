# VCHECK.RS :ribbon: :rocket: :crab:

![VCheck.rs CI](https://github.com/angeldollface/vcheck.rs/actions/workflows/yew.yml/badge.svg)

***My Rustacean implementation of my password checker, VCheck. :ribbon: :rocket: :crab:***

## ABOUT :books:

A year ago I decided to design and implement an algorithm that checks the strength of a password. The links to these implementations can be found in the section below. This repository contains the source code for my re-implementation of a web app that checks yoiur password in Rust and Yew. Enjoy. :heart_on_fire:

## LINKS :earth_americas:

- Library implementation in ECMA Script: [VIEW](https://github.com/angeldollface/vulcheck)
- Library implementation in Rust: [VIEW](https://github.com/angeldollface/flek)
- Web app implementation in Vue.js: [VIEW](https://github.com/angeldollface/vcheck)
- Dart library: [VIEW](https://angeldollface/securitycheck)

## DEPLOYED PROJECT ON GITHUB PAGES :rocket:

To view a live deployed version of this project, click here: [VIEW](https://angeldollface.art/vcheck.rs)

## USAGE :hammer:

To use *VCheck.rs*, simply type in one of the passwords you use, press the button and wait. My algorithm will tell you if your password is secure or not. The app will tell you how high you ranked according to *VCheck.rs*'s algorithm and if the score is greater than eight, the algorithm deems your password to be secure. :heart_on_fire:

## TRY THE CODE FOR YOURSELF! :inbox_tray:

You should have the following tools installed and available from the command line:

- Rust
- Git

To try *VCheck.rs* on your own machine, follow these steps:

- 1.) Install `trunk` from [crates.io](https://crates.io/crates/trunk):

```bash
cargo install trunk
```

- 2.) Clone this project's source code:

```bash
git clone https://github.com/angeldollface/vcheck.rs.git
```

- 3.) Change directory into the source code's root directory:

```bash
cd vcheck.rs
```

- 4.) Serve the app locally (This will serve the app locally on [`http://127.0.0.1:8080/vcheck.rs/`](http://127.0.0.1:8080/vcheck.rs/).):

```bash
trunk --config trunk.toml serve --release
```

- 5.) If you want to build the app into a bundle to deploy to a server, run the command below. This will produce a directory called `dist` with the bundle inside it.

```bash
trunk --config trunk.toml build --release
```

- 5.) Enjoy! :heart_on_fire:


## CHANGELOG :black_nib:

### Version 0.1.0

- Initial release.
- Upload to GitHub.
- Deployment to GitHub Pages.

## NOTE :scroll:

- *VCheck.rs :ribbon: :rocket: :crab:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.
