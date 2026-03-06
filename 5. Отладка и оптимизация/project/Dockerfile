FROM rust:latest
FROM ubuntu:latest
WORKDIR /app

COPY . .
# RUN cargo build --release
RUN apt-get -y update
RUN apt-get -y install valgrind

CMD ["sleep", "infinity"]