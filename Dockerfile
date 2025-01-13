FROM rust:1.84-alpine
WORKDIR /etc/jobtrek/sw

# Install the dependencies
RUN apk add build-base
RUN cargo install ast-grep --locked
RUN apk add --no-cache fd musl-dev

# Copy the project files
COPY sgconfig.yml sgconfig.yml
COPY ast-grep-rules ast-grep-rules
COPY Cargo.* ./
COPY src src

# Build the project
RUN cargo build --release

# Final image
FROM alpine:3.21
COPY --from=0 /etc/jobtrek/sw/target/release/sw /usr/local/bin/sw
COPY --from=0 /etc/jobtrek/sw/sgconfig.yml /etc/jobtrek/sw/sgconfig.yml
COPY --from=0 /etc/jobtrek/sw/ast-grep-rules /etc/jobtrek/sw/ast-grep-rules
COPY --from=0 /usr/local/cargo/bin/ast-grep /usr/local/bin/ast-grep
RUN apk add --no-cache fd
ENTRYPOINT ["sw"]

