FROM rust:1.73 AS planner
WORKDIR app
# We only pay the installation cost once,
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning
# the cargo-chef version with `--version X.X.X`
RUN cargo install cargo-chef
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.73 AS cacher
WORKDIR app
RUN cargo install cargo-chef
# RUN cargo install sqlx-cli
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.73 AS builder
WORKDIR app
# Copy over the cached dependencies
RUN cargo install sqlx-cli
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
# Build our application, leveraging the cached deps!
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM rust:1.73 AS runtime
WORKDIR app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl libc6 \
    # Clean up
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mudro mudro
EXPOSE 8080
# COPY --from=cacher ~/.cargo/bin/sqlx sqlx
# RUN sqlx migrate run
ENTRYPOINT ["./mudro"]