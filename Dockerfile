FROM rust:1.76 as builder
WORKDIR /usr/src/tagcm
COPY . /usr/src/tagcm
RUN cargo install --path .

FROM debian:bookworm-slim
WORKDIR /usr/src/tagcm
RUN apt-get update && apt-get install -y libc6 libc-bin && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/tagcm /usr/local/bin/tagcm

ENTRYPOINT [ "tagcm" ]
CMD [ "--help" ]

