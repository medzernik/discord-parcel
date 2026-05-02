FROM alpine:latest AS build

ARG ZIG_VER=0.16.0

RUN apk add --no-cache curl xz

WORKDIR /opt

# Map Docker arch → Zig arch naming
# https://ziglang.org/download/0.16.0/zig-aarch64-linux-0.16.0.tar.xz
RUN curl -L https://ziglang.org/download/${ZIG_VER}/zig-aarch64-linux-${ZIG_VER}.tar.xz -o zig.tar.xz && \
    tar -xvf zig.tar.xz && \
    mv zig-aarch64-linux-${ZIG_VER} zig

ENV PATH="/opt/zig:$PATH"

WORKDIR /app
COPY . .

RUN zig build -Doptimize=ReleaseSmall


FROM alpine:latest

RUN apk add --no-cache ca-certificates

COPY --from=build /app/zig-out/bin/medzernik_discord_parcel /server

CMD ["/server"]
