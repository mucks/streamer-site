FROM rust:1.34

WORKDIR /usr/src/tsquery_rs
COPY . .

RUN cargo install --path .

EXPOSE 3012

CMD ["tsquery_rs"]