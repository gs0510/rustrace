FROM rust:1.31

COPY . /Users/gargisharma/rust/projects/strace
WORKDIR /Users/gargisharma/rust/projects/strace
COPY . .


RUN cargo install --path .