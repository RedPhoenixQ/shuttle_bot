FROM rust:1.92-bullseye as builder

WORKDIR ./shuttle_bot
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
RUN cargo build --locked --release


FROM debian:bullseye-slim
ARG APP=/usr/src/app

# RUN apt-get update \
#     && apt-get install -y ca-certificates tzdata \
#     && rm -rf /var/lib/apt/lists/*

# EXPOSE 8000

ENV APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /shuttle_bot/target/release/redphoenixq-shuttle-bot ${APP}/shuttle_bot

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./shuttle_bot"]