# Start from the official Rust image
FROM rust:latest

# Install common development tools and libraries
RUN apt-get update && apt-get install -y \
    build-essential \
    vim \
    pkg-config \
    libssl-dev \
    libclang-dev \
    clang \
    cmake \
    git \
    curl \
    lldb \
    lld \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Optional: Create a non-root user for better practice
#RUN useradd -ms /bin/bash rustdev
#USER rustdev
#WORKDIR /home/rustdev/app
# Set the entry point command
CMD ["bash"]
