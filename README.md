# yocto

Yocto is a minimal key-value store built for fast and reliable data exchange between applications. It's written with an emphasis on security, speed and ease-to-use.

## Usage

You can use yocto either by manually building it from source or via Docker.

### Docker 

Run

```
docker pull yocto:latest
docker run -d -p 7001:7001 yocto
```

### Build from source

Pull the repository and execute 

```
cargo build --release
cargo install
```
