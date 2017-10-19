FROM scorpil/rust:stable

WORKDIR /rust

RUN apt-get update &&\
    apt-get install -y --no-install-recommends pkg-config libssl-dev &&\
    rm -rf /var/lib/apt/lists/*

COPY . .

# Create an empty directory to get downloads, and install zou
RUN mkdir downloads && cargo install

ENTRYPOINT ["/root/.cargo/bin/zou", "--output", "/rust/zou/downloads"]