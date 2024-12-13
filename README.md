# xrk-cli

Simple cli to read AiM `xrk` data files.

## Development

Early stages CLI, exploring possibilities.

> :warning:
> the is based around the Rust xdrk lib, which in turns wraps around `libmatlabxrk.so.0`, which is actually `libxdrk-x86_64.so`
> which means:
>  - Linux
>  - x86_64
> 
> in my case, both my M1 Macbook and FreeBSD server are utterly useless here (well, I'm building this on a bhyve vm, so there's that) 

:bulb: My build env is _Ubuntu 24.04.1 LTS_.

```bash
$ sudo apt install libxml2-dev
```

```bash
cargo build --release
```

```bash
mkdir ~/lib
ln -s ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/xdrk-1.0.0/aim/libmatlabxrk.so.0 ~/lib/
```

```bash
$ LD_LIBRARY_PATH=~/lib ./target/release/aim-reader-cli --help
Simple CLI to inspect and process XRK race data files

Usage: aim-reader-cli <FILE_PATH>

Arguments:
  <FILE_PATH>  Path to the xrk data file

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## TODO

Unknown error:

```text
I/O warning : failed to load external entity "user/profiles/units.xml"
I/O error : No such file or directory
I/O error : No such file or directory
File '**REDACTED**.xrk' loaded successfully!
```

Current workaround:
```bash
mkdir -p user/profiles
echo "<XML/>" > user/profiles/units.xml
```
