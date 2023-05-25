#####STEP 1######

FROM rust:latest AS builder

RUN update-ca-certificates

# Create user
ENV USER=axum
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY . .

RUN cargo build --release

#####STEP 2######

FROM debian:bookworm-slim

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

COPY --from=builder /app/target/release/axum-backend .
COPY .env .
#COPY html ./html
#COPY ./success.html .

USER axum:axum

CMD ["./axum-backend"]

