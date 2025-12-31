FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV USER=root

# Base packages and utilities
RUN apt update && apt install -y --no-install-recommends \
    apt-utils                       \
    bash-completion                 \
    build-essential                 \
    ca-certificates                 \
    clang-format                    \
    clang-tidy                      \
    cmake                           \
    curl                            \
    git                             \
    htop                            \
    iputils-ping                    \
    lsb-release                     \
    lsof                            \
    openssh-client                  \
    sudo                            \
    tree                            \
    unzip                           \
    wget       	                    \
    vim                             \
    xz-utils                        \
    # Add user in sudoer group
    && adduser root sudo            \
    # Cleanup
    && rm -rf /var/lib/apt/lists/*

# # Install rust
# RUN apt update && apt install -y --no-install-recommends \
#     rustc                           \
#     cargo                           \
#     # Add user in sudoer group
#     && adduser root sudo            \
#     # Cleanup
#     && rm -rf /var/lib/apt/lists/*

# Create the user
ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN userdel --remove ubuntu \
    && groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && usermod -s /bin/bash $USERNAME \
    # Add sudo support
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/localuser \
    && chmod 0440 /etc/sudoers.d/localuser

# Set the default user
USER $USERNAME
ENV USER=$USERNAME

# Install rustup for the current user
WORKDIR /home/$USERNAME
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add cargo/bin to PATH for this user
ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"

# Install rustfmt and clippy
RUN rustup component add rustfmt
RUN rustup component add clippy

# Verify installation
RUN rustc --version
RUN cargo --version
RUN rustup --version
