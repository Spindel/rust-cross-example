FROM debian

COPY rustup.sh /usr/local/bin/rustup.sh
COPY cargo-config /cargo/config

ENV CARGO_HOME=/cargo \
    PATH=/cargo/bin:$PATH \
    USER=root

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install -y curl git && \
    apt-get install -y build-essential && \
    apt-get install -y libc6-armhf-cross libc6-dev-armhf-cross gcc-arm-linux-gnueabihf && \
    apt-get install -y libdbus-1-dev libdbus-1-dev:armhf  && \
    apt-get clean

RUN bash /usr/local/bin/rustup.sh -y && \
    rustup default stable && \
    rustup target add armv7-unknown-linux-gnueabihf && \
    rustup target add x86_64-unknown-linux-gnu

WORKDIR /code
