# docker build -t vocab_vault_api .
# docker run -p 8080:8080 vocab_vault_api

FROM rust:slim-buster

WORKDIR /usr/src/app

ENV PORT 8080
ENV HOST 0.0.0.0

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["cargo", "run", "--release"]