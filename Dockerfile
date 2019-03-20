FROM rust:1.31

COPY . rustrace
WORKDIR rustrace
COPY . .


RUN cargo install --path .
