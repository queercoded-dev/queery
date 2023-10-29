####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=queery
ENV UID=10001

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

RUN cargo install sqlx-cli

RUN cargo sqlx migrate run

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bookworm-slim

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /queery

# Copy our build
COPY --from=builder /queery/target/release/queery ./

# Use an unprivileged user.
USER queery:queery

CMD ["/queery/queery"]
