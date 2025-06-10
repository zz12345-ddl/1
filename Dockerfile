
# 设置环境变量
FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive
ENV RUST_VERSION=1.75.0
ENV NODE_VERSION=20
ENV PNPM_VERSION=9.0.0

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf \
    && rm -rf /var/lib/apt/lists/*

# 安装Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_${NODE_VERSION}.x | bash - \
    && apt-get install -y nodejs

# 安装pnpm
RUN npm install -g pnpm@${PNPM_VERSION}

# 安装Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain ${RUST_VERSION}
ENV PATH="/root/.cargo/bin:${PATH}"

# 添加Rust目标平台
RUN rustup target add x86_64-pc-windows-msvc
RUN rustup target add aarch64-pc-windows-msvc
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add aarch64-unknown-linux-gnu

# 设置工作目录
WORKDIR /app

# 复制项目文件
COPY package.json pnpm-lock.yaml ./
COPY src-tauri/Cargo.toml src-tauri/Cargo.lock ./src-tauri/

# 安装依赖
RUN pnpm install

# 复制源代码
COPY . .

# 构建脚本
CMD ["pnpm", "run", "tauri", "build"]