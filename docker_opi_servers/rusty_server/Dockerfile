FROM rust AS builder

RUN mkdir /tmp/opi

COPY Cargo.toml /tmp/opi/Cargo.toml
COPY Cargo.lock /tmp/opi/Cargo.lock
COPY src /tmp/opi/src

WORKDIR /tmp/opi
RUN apt update
RUN apt install libzbar-dev -y
RUN cargo build --release

FROM ubuntu

RUN mkdir -p /bin/opi
RUN apt update
RUN apt install libzbar0 -y
COPY --from=builder /tmp/opi/target/release/rusty_server /bin/opi/

WORKDIR /bin/opi

COPY public .

RUN chmod +x rusty_server

EXPOSE 12001
EXPOSE 8080

ENTRYPOINT [ "/bin/opi/rusty_server" ]