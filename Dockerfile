FROM node:20-slim AS frontend_builder
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

COPY ./frontend/svelte-client /frontend/svelte-client

WORKDIR /frontend/svelte-client

FROM frontend_builder AS fe_prod_deps
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --prod --frozen-lockfile

FROM frontend_builder AS fe_build
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

RUN pnpm build

#######################################################################################

FROM clux/muslrust:stable AS backend_builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

RUN cargo new orderbook_trial_task

WORKDIR /orderbook_trial_task

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --features prod  --target x86_64-unknown-linux-musl --release

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/orderbook_trial_task*

RUN cargo build --features prod --target x86_64-unknown-linux-musl --release


############################################################
FROM alpine

RUN apk add bash

WORKDIR /

# COPY bins
COPY --from=backend_builder /orderbook_trial_task/target/x86_64-unknown-linux-musl/release/orderbook_trial_task usr/bin/orderbook_trial_task

# COPY web assets
COPY --from=fe_prod_deps /frontend/svelte-client/node_modules /etc/www/node_modules 
COPY --from=fe_build /frontend/svelte-client/dist /etc/www/dist


EXPOSE 3000


CMD orderbook_trial_task 
