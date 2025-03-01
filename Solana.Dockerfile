FROM solanalabs/solana:v1.11.3 as builder
RUN apt-get update \
      && apt-get -y install \
           wget \
           curl \
           build-essential \
           software-properties-common \
           lsb-release \
           libelf-dev \
           linux-headers-generic \
           pkg-config \
           curl \
          cmake \
          protobuf-compiler

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /rust/
COPY plerkle_serialization /rust/plerkle_serialization
COPY plerkle_messenger /rust/plerkle_messenger
COPY plerkle /rust/plerkle
WORKDIR /rust/plerkle
RUN cargo build

FROM solanalabs/solana:v1.11.3
COPY --from=builder /rust/plerkle/target/debug/libplerkle.so /plugin/plugin.so
COPY ./docker .
RUN chmod +x ./*.sh
ENTRYPOINT [ "./runs.sh" ]
CMD [""]
