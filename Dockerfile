################################################################################
# Create a stage for building the application.

ARG RUST_VERSION=1.78
FROM rust:${RUST_VERSION}-slim-bookworm AS build

# This ARG should be after `FROM` clause
ARG APP_NAME=banksim
WORKDIR /app

RUN apt update && apt install libssl-dev pkg-config -y

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=templates,target=templates \
    --mount=type=bind,source=migrations,target=migrations \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    set -e && \
    cargo build --release && \
    cp ./target/release/$APP_NAME /app/$APP_NAME

################################################################################
# Create a stage for building the frontend assets.

FROM node:latest AS frontend

WORKDIR /frontend

# Copy the frontend source code
COPY frontend/src /frontend/src
COPY frontend/index.html /frontend/index.html
COPY frontend/package.json /frontend/package.json
COPY frontend/public /frontend/public
COPY frontend/tsconfig.json /frontend/tsconfig.json
COPY frontend/vite.config.ts /frontend/vite.config.ts
COPY frontend/tsconfig.node.json /frontend/tsconfig.node.json

# Install dependencies and build the frontend assets
RUN npm install -g pnpm && \
    pnpm install && \
    pnpm run build

################################################################################
# Create a stage for running the application.
FROM debian:bookworm-slim AS final

RUN apt update && apt install -y ca-certificates curl

# Create a non-privileged user that the app will run under.
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

WORKDIR /app

# Copy the executable from the "build" stage.
COPY --from=build /app/$APP_NAME /app/$APP_NAME

# Copy the frontend assets from the "frontend" stage
COPY --from=frontend /frontend/dist /app/dist

# Expose the port that the application listens on.
EXPOSE 15100

# What the container should run when it is started.
CMD ["/app/banksim"]


