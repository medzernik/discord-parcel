FROM alpine:latest AS build

ARG ZIG_VER=0.16.0

RUN apk add --no-cache curl xz rust

WORKDIR /app
COPY . .

RUN cargo build --release


FROM alpine:latest

RUN apk add --no-cache ca-certificates

COPY --from=build /target/release/medzernik_discord_parcel /server

CMD ["/server"]
