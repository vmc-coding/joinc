# joinc - Frontends to BOINC clients

joinc implements frontends to [BOINC](https://boinc.berkeley.edu/) clients.

The core of joinc implements the [GUI RPC protocol](https://boinc.berkeley.edu/trac/wiki/GuiRpcProtocol).
On top of this a CLI is provided.
A GUI and/or web API is subject for the future.

It's a hobby project to learn Rust.
The project is work in progress and at the current state the API and/or CLI may change without any public notices.

## joinc consists of

- **libjoincserde**: The serde library implementing (de)serializing for the communication with the BOINC clients.
- **libjoinc**: The core library implementing the communication with the BOINC clients by abstracting it through
    [commands](https://en.wikipedia.org/wiki/Command_pattern).
- **joinccmd**: The CLI to the BOINC clients.

## dependencies

### runtime

As rust builds statically linked binaries, there aren't any known runtime dependencies at the moment.

### compiletime

- [rustc and cargo](https://www.rust-lang.org/):
    I don't know the minimum version needed, I'm using rustc/cargo v1.76.0 at the moment.
    You may want to use [rustup](https://www.rust-lang.org/tools/install) to install the toolchain.

## building
- get the source of joinc, e.g. by cloning the repo
    ```shell script
    $ git clone https://github.com/vmc-coding/joinc.git joinc
    $ cd joinc
    ```
- compile it with cargo
    ```shell script
    $ cargo build --release
    ```
  for development you may want to build in debug mode
    ```shell script
    $ cargo build
    ```
- optionally run tests
    ```shell script
    $ cargo test
    ```

## installing
There are at least two ways to install joinc:
- Use the build steps from above and use the generated binary target/release/joinccmd.
- Use cargo to install it, the binary will be installed as ${CARGO_HOME}/bin/joinccmd.

## running joinccmd
Sorry, all I have is
```shell script
$ joinccmd -h
```

When developing you may also just let cargo run it
```shell script
$ cargo run
```
