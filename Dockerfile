FROM ubuntu:22.04 AS builder

# Use bash for the shell
SHELL ["/bin/bash", "-c"]
RUN echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
  ssl-cert \
  ca-certificates \
  curl \
  apt-transport-https \
  lsb-release \
  file \
  git-core \
  build-essential \
  libssl-dev \
  libgexiv2-dev \
  pkg-config

RUN mkdir -p /opt/build
WORKDIR /opt/build
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source /root/.cargo/env && rustup +nightly default

COPY Cargo.* /opt/build/
COPY src /opt/build/src
COPY rust-toolchain.toml /opt/build

RUN source /root/.cargo/env && cargo build --bin server --release

FROM ubuntu:22.04
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
  ssl-cert \
  ca-certificates \
  libgexiv2-dev \
  libpq-dev

RUN mkdir -p /srv/fcc/static
RUN mkdir -p /srv/fcc/output/reports
WORKDIR /srv/fcc
COPY static/index.* static/
COPY --from=builder /opt/build/target/release/server ./
COPY Rocket.toml ./

COPY files/start /usr/local/bin/start
RUN chmod a+x /usr/local/bin/start

EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/start"]
