#@see: https://hub.docker.com/_/rust
FROM rust:1.67

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

#CMD ["myapp"]