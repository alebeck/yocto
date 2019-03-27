FROM rust:1.33 as build

WORKDIR /usr/src/yocto

COPY . .

RUN cargo build --release

######
FROM debian:jessie-slim

ENV YOCTO_THREADS 4
ENV YOCTO_BIND "0.0.0.0:7001"
ENV YOCTO_VERBOSE ""

WORKDIR /usr/local/bin

COPY --from=build /usr/src/yocto/target/release/yocto  ./yocto

EXPOSE 7001

RUN ls -la

CMD ["sh", "-c", "./yocto --threads ${YOCTO_THREADS} --iface ${YOCTO_BIND} ${YOCTO_VERBOSE:+--verbose}"]