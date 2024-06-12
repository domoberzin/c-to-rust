FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

RUN DEBIAN_FRONTEND=noninteractive apt-get update && DEBIAN_FRONTEND=noninteractive \
  apt-get install -y python3-dev python3-pip sudo curl git htop vim build-essential m4

RUN DEBIAN_FRONTEND=noninteractive apt-get update && DEBIAN_FRONTEND=noninteractive \
  apt-get install -y ncdu ranger tmux rsync openssh-client openssh-server

# for compiling coreutils
RUN DEBIAN_FRONTEND=noninteractive apt-get update && DEBIAN_FRONTEND=noninteractive \
  apt-get install -y autoconf automake autopoint bison gettext gperf m4 texinfo

