FROM rust:1.74
WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .
RUN cargo build

CMD ["target/debug/adv2023"]
