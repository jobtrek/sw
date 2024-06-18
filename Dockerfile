FROM rust:1.79-alpine
RUN apk add build-base
RUN cargo install ast-grep --locked
RUN apk add --no-cache fd musl-dev

WORKDIR /etc/jobtrek/sw
COPY sgconfig.yml sgconfig.yml
COPY example example
COPY ast-grep-rules ast-grep-rules
COPY Cargo.* ./

# Final build
RUN cargo build --release

FROM alpine:3.20
COPY --from=0 /etc/jobtrek/sw/target/release/sw /usr/local/bin/sw
COPY --from=0 /etc/jobtrek/sw/sgconfig.yml /etc/jobtrek/sw/sgconfig.yml
COPY --from=0 /etc/jobtrek/sw/ast-grep-rules /etc/jobtrek/sw/ast-grep-rules
COPY --from=0 /usr/local/cargo/bin/ast-grep /usr/local/bin/ast-grep
RUN apk add --no-cache fd

