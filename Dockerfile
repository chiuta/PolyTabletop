# syntax=docker/dockerfile:1
# Build stage: newest stable Rust (CLI + RPG only; GUI needs a display).
FROM rust:1-trixie AS build
WORKDIR /src
COPY . .
RUN cargo build --release -p polytabletop -p polytabletop-rpg

# Final stage: Ubuntu 26.04 (distroless has no 26.04 base; binaries need only glibc).
FROM ubuntu:26.04
COPY --from=build /src/target/release/polytabletop /src/target/release/polytabletop-rpg /usr/local/bin/
USER 65534:65534
ENTRYPOINT ["polytabletop"]
CMD ["list"]
