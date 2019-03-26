# yocto

[![Build Status](https://ci.alexbe.dev/api/badges/alebeck/yocto/status.svg)](https://ci.alexbe.dev/alebeck/yocto)

Yocto is a minimalistic key-value store built for fast and reliable state exchange between applications. It's written with an emphasis on reliability, speed and ease-to-use.

## Usage

You can use yocto either by manually building it from source or via Docker.

### Docker 

In the below snipped, replace `<host_port>` with the port you want yocto to bind to:

```
docker pull yocto:latest
docker run -d -p <host_port>:7001 yocto
```

Following environment variables can be passed:

- `YOCTO_THREADS`: Number of threads, defaults to `4`
- `YOCTO_VERBOSE`: Show debug logs


### Build from source

Pull the repository and execute 

```
cargo test -- --test-threads=1
cargo build --release
cargo install
```
