FROM debian:trixie AS build

ARG ZIG_VER=0.16.0

RUN apt update && apt install -y curl xz-utils

RUN curl https://ziglang.org/builds/zig-linux-$(uname -m)-${ZIG_VER}.tar.xz -o zig-linux.tar.xz && \
    tar xf zig-linux.tar.xz && \
    mv zig-linux-$(uname -m)-${ZIG_VER}/ /opt/zig

WORKDIR /app

COPY . .

# Using ReleaseSmall instead of ReleaseFast - because there is no CPU intensive work
# in this app, so ReleaseSmall will give a smaller binary / better use of CPU cache
RUN /opt/zig/zig build -Doptimize=ReleaseFast

FROM debian:trixie-slim

RUN apt update && apt install -y ca-certificates

COPY --from=build /app/zig-out/bin/medzernik_discord_parcel /server

CMD ["/server"]
