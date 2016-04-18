FROM kbacha/rust:1.8
MAINTAINER chewbacha@gmail.com

# Do a simple build for pushing to hub
RUN mkdir /myapp
WORKDIR /myapp
COPY . /myapp
RUN cargo build
EXPOSE 3000
