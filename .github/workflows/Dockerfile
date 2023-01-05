FROM ubuntu:22.10

ENV DEBIAN_FRONTEND=noninteractive

# Taking into account layer structure, everything should be done within one layer.
RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y clang-15 clang-tidy-15 clang-format-15 git libdrm-dev blueprint-tools libgtest-dev clang \
    llvm make python3 python3-pip wget sudo rsync lld pkg-config ninja-build && \
    pip3 install mako meson jinja2 ply pyyaml

ENV USER_HOME /github/home

RUN mkdir -pv ${USER_HOME}

# Install aospless package (produced by GloDroid/aospext)
RUN wget -P /github https://github.com/GloDroid/mm-radio/files/10360721/aospless_mm_radio.tar.xz.zip && \
    cd /github && mv aospless_mm_radio.tar.xz.zip aospless_mm_radio.tar.xz && tar xf aospless_mm_radio.tar.xz && \
    ln -s ../workspace/ /github/aospless/src

# Create project path
RUN mkdir -pv /github/workspace
WORKDIR /github/workspace

RUN git config --global user.name "FIRST_NAME LAST_NAME" && git config --global user.email "MY_NAME@example.com"

# Run CI:
CMD cd /github/workspace && \
    git config --global --add safe.directory /github/workspace && \
    git config --global -l && \
    make -C ../aospless all && \
    .ci/.gitlab-ci-checkcommit.sh && \
    make -f .ci/Makefile -j$(nproc) && \
    mkdir -p install/arm64 && \
    cp -r ../aospless/install/* install/arm64 && \
    echo "\n\e[32m --- SUCCESS ---\n"

