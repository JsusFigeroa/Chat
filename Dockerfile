FROM rust:1.94

WORKDIR /Chat

COPY . .

RUN cargo build

EXPOSE 8080 
