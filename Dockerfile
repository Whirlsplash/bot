# # NOTICE
# This Dockerfile is *probably* not stable as of 2021. 04. 25.
#
# Can you use it? Yes, but you probably shouldn't.

FROM rustlang/rust:nightly-slim AS build

WORKDIR /src/bot

COPY . .

RUN cargo build --release

FROM ubuntu:18.04

COPY --from=build /src/bot/target/release/bot /usr/local/bin/bot

CMD ["usr/local/bin/bot"]
