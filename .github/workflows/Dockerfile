FROM ubuntu:22.10

ENV DEBIAN_FRONTEND=noninteractive

# Taking into account layer structure, everything should be done within one layer.
RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y clang-15 clang-tidy-15 clang-format-15 git libdrm-dev blueprint-tools libgtest-dev clang \
    llvm make python3 python3-pip wget sudo rsync lld pkg-config ninja-build curl && \
    pip3 install mako meson jinja2 ply pyyaml

ENV USER_HOME /github/home

RUN mkdir -pv ${USER_HOME}

# Install aospless package (produced by GloDroid/aospext)
RUN wget -P /github https://github.com/GloDroid/mm-radio/files/10420551/aospless_mm_radio.tar.gz.zip && \
    cd /github && mv aospless_mm_radio.tar.gz.zip aospless_mm_radio.tar.gz && tar xf aospless_mm_radio.tar.gz && \
    ln -s ../workspace/ /github/aospless/src

# Create project path
RUN mkdir -pv /github/workspace
WORKDIR /github/workspace

RUN git config --global user.name "FIRST_NAME LAST_NAME" && git config --global user.email "MY_NAME@example.com"

# Run CI:
CMD cd /github/workspace && \
    curl https://sh.rustup.rs -sSf | bash -s -- -y && \
    . ${USER_HOME}/.cargo/env && \
    rustup target add aarch64-linux-android && \
    git config --global --add safe.directory /github/workspace && \
    git config --global -l && \
    export RUSTFLAGS="-D warnings" && cargo check && cargo test && cargo fmt --check && cargo clippy && \
    make -C ../aospless install && \
    .ci/.gitlab-ci-checkcommit.sh && \
    make -f .ci/Makefile -j$(nproc) && \
    mkdir -p install/arm64 && \
    cp -r ../aospless/install/* install/arm64 && \
    echo "\n\e[32m --- SUCCESS ---\n"
