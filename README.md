# Reversing Rust Binaries: One step beyond strings - REcon 2024 Workshop

This repository contains the supplementary files for the [_Reversing Rust Binaries: One step beyond strings_ workshop at REcon 2024](https://cfp.recon.cx/recon2024/talk/QCA37X/), presented on June 28, 2024.

During the presentation, we will be building and reversing a very simple Rust binary - a benign downloader. The source code for the downloader is inside this repository, in the `simple-downloader/` folder.

Read below for the pre-workshop setup instructions!

## Automatic setup: Pre-configured Ubuntu VM

For easy setup, you can download a preconfigured Ubuntu VM which has all of the following:

- The Rust toolchain (Rust version 1.77.1)
- Visual Studio Code, with the [`rust-analyzer` extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) preinstalled (for autocomplete and documentation tooltips)
- The code in the `simple-downloader/` folder of this repository
- Ghidra (version 11.0.3)
- A Ghidra project with prebuilt Linux versions of the `simple-downloader` binary

You can download the VM file (a .OVA file) here: https://drive.google.com/file/d/1pM1MwQf4Ccjt-KokbV1RFh8C9tv48jrt/view?usp=sharing

You can then import this OVA file into your VM software (e.g. VMware Workstation, VMWare Fusion, VirtualBox, etc.)

The main account on the VM has username `user` and password `user`.

## Manual setup

If you would like to do this workshop from the comfort of your own machine, you will need the following:

1) A machine that can build a Rust binary. We'll be pulling dependencies to build the binary, so that machine should have internet access.
    - For instructions on how to set up the Rust toolchain for building your binary, see the _Installing the Rust Toolchain_ section below.
2) A machine that can run the Rust binary you built. We'll be building a benign downloader in this workshop, so that machine should ideally also have internet access.
    - The [provided source code](https://github.com/cxiao/rust-reversing-workshop-recon-2024/blob/main/simple-downloader/src/main.rs) downloads and runs a shell script for macOS / Linux systems (`https://sh.rustup.rs/rustup-init.sh`); however, the code is very easily adaptable for Windows as well.
3) Your preferred reverse engineering tool, for reversing the Rust binary you built!
    - A Ghidra project with some prebuilt Linux versions of the `simple-downloader` binary is provided here: [prebuilt-files/rust-reversing-workshop-2024_2024_05_17.gar](https://github.com/cxiao/rust-reversing-workshop-recon-2024/blob/main/prebuilt-files/rust-reversing-workshop-2024_2024_05_17.gar)
    - The prebuilt Linux `simple-downloader` binaries in the Ghidra project are also available separately:
        - [`simple-downloader-release-build-stripped`](https://github.com/cxiao/rust-reversing-workshop-recon-2024/blob/main/prebuilt-files/simple-downloader-release-build-stripped)
        - [`simple-downloader-release-build-unstripped`](https://github.com/cxiao/rust-reversing-workshop-recon-2024/blob/main/prebuilt-files/simple-downloader-release-build-unstripped)

### Installing the Rust Toolchain

The official toolchain setup instructions are at https://rustup.rs/. Follow the instructions there; the page will display specific instructions for your operating system.

Check, after installation, that you can run both the newly installed `rustup` and `cargo` tools:

```
$ rustup --version
rustup 1.27.0 (bbb9276d2 2024-03-08)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.77.1 (7cf61ebde 2024-03-27)`
```

```
$ cargo --version
cargo 1.77.1 (e52e36006 2024-03-26)
```

We'll be using Rust version 1.77.1 in this workshop, so install that version, and switch your toolchain to that version as the default for all builds:

```
rustup install 1.77.1
rustup default 1.77.1
```

#### Linux users: Ensure you have build tools

If you're building on Windows or macOS, you can skip this step.

If you're building on Linux: Parts of this build rely on `gcc`, `ld`, `pkg-config`, and the OpenSSL development headers. On Ubuntu, you can install these with

```
sudo apt install build-essential
sudo apt install pkg-config
sudo apt install libssl-dev
```
