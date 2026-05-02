FROM debian:trixie AS build

ARG ZIG_VER=0.16.0

RUN apt update && apt install -y curl xz-utils

RUN curl https://ziglang.org/download/0.16.0/zig-aarch64-linux-0.16.0.tar.xz -o zig-linux.tar.xz && \
    tar xvf zig-linux.tar.xz && \
    mv zig-$(uname -m)-linux-${ZIG_VER}/ /opt/zig

WORKDIR /app

COPY . .

# Using ReleaseSmall instead of ReleaseFast - because there is no CPU intensive work
# in this app, so ReleaseSmall will give a smaller binary / better use of CPU cache
RUN /opt/zig/zig build -Doptimize=ReleaseSmall

FROM debian:trixie-slim

RUN apt update && apt install -y ca-certificates

COPY --from=build /app/zig-out/bin/medzernik_discord_parcel /server

CMD ["/server/medzernik_discord_parcel"]
