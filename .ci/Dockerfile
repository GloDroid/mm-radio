FROM ubuntu:22.10

ENV DEBIAN_FRONTEND=noninteractive

ENV PATH="/home/user/bin:${PATH}"

# Taking into account layer structure, everything should be done within one layer.
RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y clang-15 clang-tidy-15 clang-format-15 git libdrm-dev blueprint-tools libgtest-dev clang \
    llvm make python3 python3-pip wget sudo rsync lld pkg-config ninja-build && \
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

# Install aospless package (produced by GloDroid/aospext)
RUN wget -P ${USER_HOME} https://github.com/GloDroid/mm-radio/files/10360721/aospless_mm_radio.tar.xz.zip && \
    cd ${USER_HOME} && mv aospless_mm_radio.tar.xz.zip aospless_mm_radio.tar.xz && \
    sha256sum aospless_mm_radio.tar.xz && \
    (echo 5365466cee98abb8b4b80b06fb1096912ec83d39b1c65e5cd6ecf3d97dcaffe5 aospless_mm_radio.tar.xz | sha256sum --check) && \
    tar xf aospless_mm_radio.tar.xz && \
    ln -s ../mm-radio/ ${USER_HOME}/aospless/src

# Create project path
RUN mkdir -pv ${USER_HOME}/mm-radio
WORKDIR ${USER_HOME}/mm-radio

RUN git config --global user.name "FIRST_NAME LAST_NAME" && git config --global user.email "MY_NAME@example.com"

CMD [ "/bin/bash" ]
