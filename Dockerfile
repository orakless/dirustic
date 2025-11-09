FROM rust:1.91 as builder

RUN apt update 
RUN apt install -y libopus-dev

WORKDIR /app
COPY . .

RUN cargo build --release 

FROM debian:trixie-slim AS runner

WORKDIR /app 

RUN apt update
RUN apt install -y wget libopus-dev

RUN wget https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux -O /usr/local/bin/yt-dlp && chmod a+x /usr/local/bin/yt-dlp

COPY --from=builder /app/target/release/dirustic /app/dirustic

RUN ls /app

CMD ["/app/dirustic"]

