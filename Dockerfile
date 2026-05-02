FROM ziglang/zig:latest as builder
WORKDIR /app

COPY . .
RUN zig build -Drelease-fast

FROM scratch
COPY --from=builder /app/zig-out/bin/medzernik_discord_parcel /medzernik_discord_parcel
CMD [“/medzernik_discord_parcel”]
