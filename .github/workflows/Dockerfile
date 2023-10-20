FROM ubuntu:23.10

ENV DEBIAN_FRONTEND=noninteractive

# Taking into account layer structure, everything should be done within one layer.
RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y clang-17 clang-tidy-17 git clang llvm make wget sudo rsync lld curl

ENV USER_HOME /github/home

RUN mkdir -pv ${USER_HOME}

# Install aospless package (produced by GloDroid/aospext)
RUN wget -P /github https://github.com/GloDroid/mm-radio/files/10724455/aospless_mm_radio.tar.gz.zip && \
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
    rustup default nightly && rustup target add aarch64-linux-android && \
    git config --global --add safe.directory /github/workspace && \
    git config --global -l && \
    export RUSTFLAGS="-D warnings" && cargo check && cargo test && cargo fmt --check && cargo clippy && \
    make -C ../aospless install && \
    .ci/.gitlab-ci-checkcommit.sh && \
    mkdir -p install/arm64 && \
    cp -r ../aospless/install/* install/arm64 && \
    echo "\n\e[32m --- SUCCESS ---\n"
