FROM rust:1.91-alpine
WORKDIR /etc/jobtrek/sw

# Install the dependencies
RUN apk add build-base
RUN apk add --no-cache fd musl-dev

# Copy the project files
COPY Cargo.* ./
COPY src src

# Build the project
RUN cargo build --release

# Final image
FROM alpine:3.23
COPY --from=0 /etc/jobtrek/sw/target/release/sw /usr/local/bin/sw
RUN apk add --no-cache fd
ENTRYPOINT ["sw"]

