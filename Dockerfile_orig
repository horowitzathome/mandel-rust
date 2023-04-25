# syntax=docker/dockerfile:1.3
FROM rust:1.65.0 AS builder

ENV IMAGE_NAME=mandel-rust

ARG TARGETPLATFORM
ARG TARGET
ARG RUSTARGS

WORKDIR /root

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=/root/target,id=${TARGETPLATFORM} \
    cargo build --release --target ${TARGET} ${RUSTARGS} && \
    cargo strip && \
    mv /root/target/${TARGET}/release/${IMAGE_NAME} /root

FROM gcr.io/distroless/static:nonroot
#FROM gcr.io/distroless/base:debug

ENV IMAGE_NAME=mandel-rust

WORKDIR /${IMAGE_NAME}

# Copy the missing files from the Rust image to the Distroless image
COPY --from=builder /lib/x86_64-linux-gnu/libgcc_s.so.1 /usr/lib/x86_64-linux-gnu/

#COPY --from=builder /bin/dash /mandel-rust/sh
#COPY --from=builder /bin/dash /mandel-rust/dash
#COPY --from=builder /bin/bash /mandel-rust/bash
#COPY --from=builder /bin/ls /mandel-rust/ls
COPY --from=builder /bin/ /mandel-rust/

# RUN /bin/ls -alF /bin

# Copy our build
COPY --from=builder /root/${IMAGE_NAME} /${IMAGE_NAME}/${IMAGE_NAME}
EXPOSE 8080
#ENTRYPOINT ["/mandel-rust/mandel-rust"]
ENTRYPOINT ["/mandel-rust/sh"]