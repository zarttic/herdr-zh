#!/bin/sh
set -e

REPO="zarttic/herdr-zh"
BINARY="herdr"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# 检测平台和架构
detect_platform() {
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)  os="linux" ;;
        Darwin) os="darwin" ;;
        MINGW*|MSYS*|CYGWIN*) os="windows" ;;
        *) echo "❌ 不支持的操作系统: $os"; exit 1 ;;
    esac

    case "$arch" in
        x86_64|amd64)  arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *) echo "❌ 不支持的架构: $arch"; exit 1 ;;
    esac

    if [ "$os" = "windows" ]; then
        echo "${arch}-pc-windows-msvc"
    else
        echo "${arch}-unknown-linux-musl"
        [ "$os" = "darwin" ] && echo "${arch}-apple-darwin"
    fi
}

# 获取最新版本号
get_latest_version() {
    curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | head -1 | cut -d '"' -f 4
}

# 下载并安装
install() {
    platform=$(detect_platform)
    version=$(get_latest_version)

    if [ -z "$version" ]; then
        echo "❌ 无法获取最新版本，请检查网络连接"
        exit 1
    fi

    echo "📦 herdr-zh $version"
    echo "🖥️  平台: $platform"

    # 构建下载文件名
    case "$platform" in
        *linux*)   filename="${BINARY}-linux-${arch}" ;;
        *darwin*)  filename="${BINARY}-darwin-${arch}" ;;
        *windows*) filename="${BINARY}-windows-${arch}.exe" ;;
    esac

    url="https://github.com/${REPO}/releases/download/${version}/${filename}"

    echo "⬇️  下载: $url"

    # 下载到临时目录
    tmp_dir=$(mktemp -d)
    trap 'rm -rf "$tmp_dir"' EXIT

    curl -sL "$url" -o "$tmp_dir/$BINARY"
    chmod +x "$tmp_dir/$BINARY"

    # 安装
    if [ -w "$INSTALL_DIR" ]; then
        mv "$tmp_dir/$BINARY" "$INSTALL_DIR/$BINARY"
    else
        echo "🔑 需要 sudo 权限安装到 $INSTALL_DIR"
        sudo mv "$tmp_dir/$BINARY" "$INSTALL_DIR/$BINARY"
    fi

    echo "✅ 安装完成！运行 'herdr' 开始使用"
    echo ""
    echo "📖 文档: https://github.com/${REPO}"
    echo "🔗 原项目: https://github.com/ogulcancelik/herdr"
}

install
