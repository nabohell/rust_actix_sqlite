
FROM ubuntu:20.04 as cargo-build

ENV TZ=Europe/Paris
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Update default packages
RUN apt-get -qq update

# Get Ubuntu packages
RUN apt-get install -y -q \
    build-essential \
    openssl \
    make \
    cmake \
    pkg-config \
    libssl-dev \
    libpq-dev \
    curl

# Get Rust; NOTE: using sh for better compatibility with other base images
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/app


#COPY files
COPY . ./

# run migration
RUN cargo install sqlx-cli
RUN sqlx mig run 

# Build
RUN cargo test
RUN cargo build --release

EXPOSE 8080
ENTRYPOINT [ "sh", "-c", "cargo run" ]






