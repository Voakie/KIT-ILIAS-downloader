FROM rust as builder
WORKDIR /home/kit-ilias-downloader
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/KIT-ILIAS-downloader /usr/local/bin/downloader
COPY ./start.sh /usr/start-downloader.sh

ENV USERNAME a
ENV PASSWORD a
ENV SYNC_URL a
ENV RATE 8
ENV EXTRA_ARGS a

CMD ["sh", "/usr/start-downloader.sh"]
