# zou
A simple and fast download accelerator, written in Rust

Zou is a [Snatch](https://github.com/derniercri/snatch) fork by [@k0pernicus](https://github.com/k0pernicus). Snatch is a fast and interruptable download accelerator, written in Rust, from [@k0pernicus](https://github.com/k0pernicus) and [@Jean-Serge](https://github.com/Jean-Serge).

## Current features

* **Simple**: a command line tool to manage easily your downloads ;
* **Fast**: multithreading support.

**NOTE**: _Zou_ is on _alpha_ version. This version runs well on remote contents with a length known **before** the download (with the `content-length` header from the server response).

## Goal

Let's build a better `wget` (in Rust)!

## Installation

1. Install Rust and Cargo using [rustup](https://www.rustup.rs/) ;
2. You can download two versions of _Zou_ :  
  * the latest build from [crates.io](https://crates.io/): `cargo install zou` ;
  * the last commit version from Github: `cargo install --git https://github.com/k0pernicus/zou.git --branch devel` ;
3. Enjoy !

## Usage

```
Zou 0.1.0
Zou, a simple and fast download accelerator, written in Rust.

USAGE:
    zou [FLAGS] [OPTIONS] <url>

FLAGS:
    -d, --debug      Active the debug mode
        --force      Assume Yes to all queries and do not prompt
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>          The local file to save the remote content file
    -t, --threads <threads>    Threads which can use to download

ARGS:
    <url>
```

## File examples

* [A simple PDF file](http://www.cbu.edu.zm/downloads/pdf-sample.pdf)
* [Big Buck Bunny](http://distribution.bbb3d.renderfarming.net/video/mp4/bbb_sunflower_1080p_60fps_stereo_abl.mp4), a big free mp4 file
* [The cat DNA](http://hgdownload.cse.ucsc.edu/goldenPath/felCat8/bigZips/felCat8.fa.gz), a big .gz file
* [A big PDF file from Princeton](http://scholar.princeton.edu/sites/default/files/oversize_pdf_test_0.pdf)

## Contributing

You want to contribute to _Zou_ ?
Here are a few ways you can help us out :

* test and deploy automatically the beta and stable binaries (with Travis for example),
* improve the documentation,
* improve the CLI,
* add new features (please to see our issues),
* report bugs.

If you want to create a pull request, this is the procedure to make it great:

* create an issue to explain the problem you encountered (except for typo),
* fork the project,
* create a local branch to make changes (from our `devel` branch),
* test your changes,
* create a pull request (please compare it with our `devel` branch),
* explain your changes,
* submit !

Thank you for your interest in contributing to _Zou_ ! :-D

## FAQ

* Why this fork ?
Snatch has been developed when I was a R&D engineer at [DernierCri](https://derniercri.io) - it was a "just-for-fun" project.
Today, I do not work anymore in/with this startup, and I want to experiment something different.

* Libraries cannot be build
Please go check if you are using the latest version of `rustc` (stable), running `rustup update`.

* `fatal error: 'openssl/hmac.h' file not found`
If you are on a GNU/Linux distribution (like Ubuntu), please install `libssl-dev`.
If you are on macOS, please install `openssl` and check your OpenSSL configuration:

```
brew install openssl
export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
export DEP_OPENSSL_INCLUDE=`brew --prefix openssl`/include
```
