FROM ubuntu:latest
FROM rust:1.93
WORKDIR /app

COPY . .
RUN apt-get -y update
RUN apt-get -y install valgrind curl linux-perf && \ 
    apt-get install -y linux-tools-generic || true

# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# ENV PATH="/root/.cargo/bin:${PATH}"

# RUN cargo build --release
CMD ["sleep", "infinity"]