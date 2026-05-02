FROM alpine:latest AS build

ARG ZIG_VER=0.16.0

RUN apk add --no-cache zig curl gcc

WORKDIR /app

COPY . .

# Using ReleaseSmall instead of ReleaseFast - because there is no CPU intensive work
# in this app, so ReleaseSmall will give a smaller binary / better use of CPU cache
RUN zig build -Doptimize=ReleaseSmall -Dtarget=x86_64-linux-musl

FROM alpine:latest

FROM scratch

COPY --from=build /app/zig-out/bin/medzernik_discord_parcel /server

CMD ["/server"]
