FROM rust:1.74-bookworm as build

# create a new empty shell project.
RUN USER=root cargo new --bin cyb
WORKDIR /cyb

# create project structure directories.
RUN mkdir --parents api database/entity database/migration frontend modules

# copy over cargo manifests
COPY ./Cargo.lock                       ./Cargo.lock
COPY ./Cargo.toml                       ./Cargo.toml
COPY ./api/Cargo.toml                   ./api/Cargo.toml
COPY ./database/Cargo.toml              ./database/Cargo.toml
COPY ./database/entity/Cargo.toml       ./database/entity/Cargo.toml
COPY ./database/migration/Cargo.toml    ./database/migration/Cargo.toml
COPY ./frontend/Cargo.toml              ./frontend/Cargo.toml
COPY ./modules/Cargo.toml               ./modules/Cargo.toml

# copy project source tree.
COPY ./src                      ./src
COPY ./api/src                  ./api/src
COPY ./database/src             ./database/src
COPY ./database/tests           ./database/tests
COPY ./database/entity/src      ./database/entity/src
COPY ./database/migration/src   ./database/migration/src
COPY ./frontend/src             ./frontend/src
COPY ./modules/src              ./modules/src

# build for release
RUN cargo build --release

# our final base
FROM debian:bookworm-slim

# copy the build artifact from the build stage
COPY --from=build /cyb/target/release/cyb .

# set the startup command to run your binary
CMD ["./cyb"]
