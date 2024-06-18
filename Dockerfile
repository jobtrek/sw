FROM rust:1.79-alpine
RUN apk add build-base
RUN cargo install ast-grep --locked
RUN apk add fd

FROM alpine:3.20

COPY --from=0 /usr/local/cargo/bin/ast-grep /usr/local/bin/ast-grep
COPY --from=0 /usr/bin/fd /usr/local/bin/fd

WORKDIR /app
COPY Cargo.* ./
COPY ast-grep-rules ast-grep-rules
COPY sgconfig.yml sgconfig.yml

