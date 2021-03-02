ARG VERSION

FROM centos:${VERSION}

# Needs to be defined after FROM
ARG USERNAME

RUN useradd -ms /bin/bash -G wheel ${USERNAME}
RUN dnf install -y \
    git \
    gcc \
    make
# Install all the Rust-specific packages
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > ~/install-rust.sh && \
    chmod a+x ~/install-rust.sh && \
    ~/install-rust.sh -y && \
    /root/.cargo/bin/rustup component add rust-analysis --toolchain stable-x86_64-unknown-linux-gnu && \
    /root/.cargo/bin/rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu && \
    /root/.cargo/bin/rustup component add rls --toolchain stable-x86_64-unknown-linux-gnu
# Move cargo directory to user directory
RUN cp -r ~/.cargo /home/${USERNAME} && \
    chown -R ${USERNAME}. /home/${USERNAME}/.cargo && \
    cp -r ~/.rustup /home/${USERNAME} && \
    chown -R ${USERNAME}. /home/${USERNAME}/.rustup && \
    tail -1 ~/.bashrc >> /home/${USERNAME}/.bashrc && \
    tail -1 ~/.bash_profile >> /home/${USERNAME}/.bash_profile && \
    tail -1 ~/.profile >> /home/${USERNAME}/.profile
USER ${USERNAME}
