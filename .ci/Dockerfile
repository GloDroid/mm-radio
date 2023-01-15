FROM ubuntu:22.10

ENV DEBIAN_FRONTEND=noninteractive

ENV PATH="/home/user/bin:${PATH}"

# Taking into account layer structure, everything should be done within one layer.
RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y clang-15 clang-tidy-15 clang-format-15 git libdrm-dev blueprint-tools libgtest-dev clang \
    llvm make python3 python3-pip wget sudo rsync lld pkg-config ninja-build curl && \
    pip3 install mako meson jinja2 ply pyyaml

ENV RUN_USER user
ENV RUN_UID 1000

ENV USER_HOME /home/user

RUN mkdir -pv ${USER_HOME}
# Create new user
RUN adduser \
    --gecos 'Build User' \
    --shell '/usr/bin/bash' \
    --uid ${RUN_UID} \
    --disabled-login \
    --disabled-password ${RUN_USER} \
    && adduser ${RUN_USER} sudo

RUN chown -R ${RUN_USER}:${RUN_USER} ${USER_HOME} && chmod -R 775 ${USER_HOME}

# Ensure sudo group users are not
# asked for a password when using
# sudo command by ammending sudoers file
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> \
/etc/sudoers

# Pass control to a newly created user
USER ${RUN_USER}

# install rustup
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="${USER_HOME}/.cargo/bin:${PATH}"
RUN rustup target add aarch64-linux-android

# Install aospless package (produced by GloDroid/aospext)
RUN wget -P ${USER_HOME} https://github.com/GloDroid/mm-radio/files/10420551/aospless_mm_radio.tar.gz.zip && \
    cd ${USER_HOME} && mv aospless_mm_radio.tar.gz.zip aospless_mm_radio.tar.gz && \
    sha256sum aospless_mm_radio.tar.gz && \
    (echo ada97e911501ea3a39fcb484b6f0fbfca1810a72d5850244fd8d42ffc9787994 aospless_mm_radio.tar.gz | sha256sum --check) && \
    tar xf aospless_mm_radio.tar.gz && \
    ln -s ../mm-radio/ ${USER_HOME}/aospless/src

# Create project path
RUN mkdir -pv ${USER_HOME}/mm-radio
WORKDIR ${USER_HOME}/mm-radio

RUN git config --global user.name "FIRST_NAME LAST_NAME" && git config --global user.email "MY_NAME@example.com"

CMD [ "/bin/bash" ]
