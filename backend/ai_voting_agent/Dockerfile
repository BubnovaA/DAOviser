FROM rust:1.82.0 AS builder

WORKDIR /usr/src/ai_voting_agent

COPY . .

RUN cargo build --release --bin ai_voting_agent

FROM rust:1.82.0

COPY --from=builder /usr/src/ai_voting_agent/target/release/ai_voting_agent /usr/local/bin/

COPY .env .env

EXPOSE 8080

CMD ["ai_voting_agent"]