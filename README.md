# yocto

[![Build Status](https://cloud.drone.io/api/badges/alebeck/yocto/status.svg)](https://cloud.drone.io/alebeck/yocto)
[![](https://images.microbadger.com/badges/version/alebeck/yocto.svg)](https://hub.docker.com/r/alebeck/yocto)
[![](https://images.microbadger.com/badges/image/alebeck/yocto.svg)](https://hub.docker.com/r/alebeck/yocto)

Yocto is a minimalistic key-value store built for fast and reliable state exchange between applications. It's written with an emphasis on reliability, speed and ease-to-use.

## Features

- Uses a concurrent hash map as main data structure to allow multiple threads. Blocks only if the same bucket is accessed by at least one write operation.
- Allows `get`, `insert`, `remove` and `clear` operations. More to come.
- Can be deployed seamlessly with Docker.

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
- `YOCTO_BIND`: IP address and port to bind to inside the docker image, defaults to `0.0.0.0:7001`
- `YOCTO_VERBOSE`: Show debug logs, default `false`


### Build from source

Pull the repository and execute 

```
cargo test -- --test-threads=1
cargo build --release
cargo install
```
