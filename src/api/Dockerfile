FROM ubuntu:20.04
USER root

WORKDIR /srv/src/api

RUN apt-get -y update \
 && DEBIAN_FRONTEND=noninteractive apt-get -yq install curl apt-utils gcc g++

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
 && . /root/.cargo/env \
 && cargo install cargo-watch

COPY src /srv/src/api/src
COPY Cargo.toml entrypoint.sh /srv/src/api/

RUN . /root/.cargo/env && cargo build

ENTRYPOINT /srv/src/api/entrypoint.sh
