####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN update-ca-certificates

RUN apt install pkg-config libfreetype6-dev libfontconfig1-dev

# Create appuser
ENV USER=queery
ENV UID=10001

ARG SQLX_OFFLINE=true

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /queery

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libfontconfig1

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /queery

# Copy our build
COPY --from=builder /queery/target/release/queery /usr/bin/queery

# Use an unprivileged user.
USER queery:queery

CMD ["queery"]
