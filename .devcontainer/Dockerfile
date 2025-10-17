FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

USER root

RUN export DEBIAN_FRONTEND=noninteractive \
    && apt-get update \
    && apt-get -y install build-essential swig python3-dev libedit-dev libncurses5-dev libxml2-dev \
    && apt-get -y install cmake \
    && apt-get -y install ninja-build \
    && apt-get -y install lld

ADD build_lldb.sh install_lldb.sh /home/vscode/
# NOTE: This should be run after the container is created as it is huge
# RUN mkdir /opt/llvm-project \
#     && /home/vscode/build_lldb.sh \
#     && chown vscode:rustlang /opt/llvm-project

# RUN /home/vscode/install_lldb.sh

USER vscode
