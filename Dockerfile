#### Rust
### Planner
FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 as planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Create appuser
ENV USER=appuser
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

### Cacher
FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

### Builder
FROM rust:1.53.0 as builder
WORKDIR app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --bin shopping-list-server

#### NPM

FROM node:16.4.2 as site

WORKDIR app
COPY shopping-list-frontend .

RUN npm ci
RUN npm run build

#### Runtime
FROM debian:buster-slim as runtime
WORKDIR /usr/local/bin

RUN apt-get update
RUN apt-get install openssl -y

# Import from planner.
COPY --from=planner /etc/passwd /etc/passwd
COPY --from=planner /etc/group /etc/group
USER appuser:appuser

COPY --from=builder /app/target/release/shopping-list-server app
COPY --from=site /app/build site

COPY Rocket.toml Rocket.toml

EXPOSE 8000

ENTRYPOINT ["app"]
