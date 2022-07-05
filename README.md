# ⚙️ cobalt

cobalt is a minimal, high performance reverse proxy written in rust using tokio. It's the greatly improved descendant of the earlier [pine](https://github.com/blobcode/pine).
It focuses on simplicity and speed, with >1ms of latency added to most requests. It also feature massively improved performance, with benchmarks on an `i5-1135` showing an
average maximum throughput of ~100,000 rps.

## installation

to install, you'll need `cargo` and `git` installed.

You can then run

```
cargo install --git https://github.com/blobcode/cobalt.git
```

## getting started

to get up and running, just install and run `cobalt -c <path to config file>`.
Please note that cobalt will need to be placed behind ssl termination, as it works with http only.

## config

cobalt is designed to be minimal and feature a small amount of config.
An example config file can be found in [`cobalt.toml`](./cobalt.toml).

---

written with ❤️ by [blobcode](https://blobco.de)
