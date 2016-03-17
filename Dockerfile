FROM ubuntu:wily
MAINTAINER chewbacha@gmail.com

# Required packages
RUN apt-get update
RUN apt-get install -y curl
RUN apt-get install -y openssl libssl-dev
RUN apt-get install -y file
RUN apt-get install -y wget
RUN apt-get install -y gcc

# Install rust
WORKDIR /tmp
RUN wget https://static.rust-lang.org/dist/rust-1.7.0-x86_64-unknown-linux-gnu.tar.gz
RUN tar xvzf rust-1.7.0-x86_64-unknown-linux-gnu.tar.gz
WORKDIR rust-1.7.0-x86_64-unknown-linux-gnu
RUN sh install.sh

# Do a simple build for pushing to hub
RUN mkdir /myapp
WORKDIR /myapp
COPY . /myapp
RUN cargo build
EXPOSE 3000

ENTRYPOINT ["cargo", "run"]
