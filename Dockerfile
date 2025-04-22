FROM rust:latest as builder
WORKDIR /builder
COPY . .
RUN cargo install --path .

FROM debian:latest
USER root
WORKDIR /root/
RUN apt-get update
RUN apt-get install ca-certificates curl gnupg lsb-release git -y

COPY --from=builder /usr/local/cargo/bin/git-sync /usr/local/bin/git-sync

CMD ["git-sync"]

