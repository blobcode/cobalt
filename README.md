# ⚙️ cobalt

cobalt is a simple, minimal reverse proxy in rust using tokio. It's the greatly improved descendant of the earlier [pine](https://github.com/blobcode/pine).
It focuses on simplicity and speed, with >1ms of latency added to most requests.

## installation

to install, you'll need `cargo` and `git` installed.

```
cargo install --git https://github.com/blobcode/cobalt.git
```

## getting started

to get up and running, just install and run `cobalt -c <path to config file>`.

## config

an example config can be found in [`cobalt.toml`](./cobalt.toml).

---

written with ❤️ by [blobcode](https://blobco.de)
