#!/usr/bin/env bash
# ==============================================================================
#  jpcargo インストールスクリプト
# ==============================================================================

set -e

# 色の定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

REPO="sha256san/jpcargo"
INSTALL_DIR="${CARGO_HOME:-$HOME/.cargo}/bin"

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD} jpcargo インストール${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# 1. OS & アーキテクチャ検出
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        OS_TARGET="unknown-linux-gnu"
        ;;
    Darwin)
        OS_TARGET="apple-darwin"
        ;;
    *)
        OS_TARGET=""
        ;;
esac

case "$ARCH" in
    x86_64|amd64)
        ARCH_TARGET="x86_64"
        ;;
    aarch64|arm64)
        ARCH_TARGET="aarch64"
        ;;
    *)
        ARCH_TARGET=""
        ;;
esac

mkdir -p "$INSTALL_DIR"
INSTALLED=false

# 2. 事前ビルド済みバイナリのダウンロード
if [ -n "$OS_TARGET" ] && [ -n "$ARCH_TARGET" ]; then
    TARGET="${ARCH_TARGET}-${OS_TARGET}"
    ARCHIVE_NAME="jpcargo-${TARGET}.tar.gz"
    DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ARCHIVE_NAME}"

    echo -e "  環境: ${BOLD}${TARGET}${NC}"
    echo -n "  バイナリ取得中... "

    TMP_DIR="$(mktemp -d)"
    trap 'rm -rf "$TMP_DIR"' EXIT

    if curl -fL -sS "$DOWNLOAD_URL" -o "$TMP_DIR/$ARCHIVE_NAME" 2>/dev/null; then
        if tar -xzf "$TMP_DIR/$ARCHIVE_NAME" -C "$TMP_DIR" 2>/dev/null; then
            if [ -f "$TMP_DIR/jpcargo" ]; then
                rm -f "$INSTALL_DIR/jpcargo" 2>/dev/null || true
                cp -f "$TMP_DIR/jpcargo" "$INSTALL_DIR/jpcargo"
                chmod +x "$INSTALL_DIR/jpcargo"
                echo -e "${GREEN}完了${NC}"
                INSTALLED=true
            fi
        fi
    fi

    if [ "$INSTALLED" = false ]; then
        echo -e "${YELLOW}ローカルビルドに切り替えます${NC}"
    fi
fi

# 3. ソースからのビルド（フォールバック）
if [ "$INSTALLED" = false ]; then
    echo ""
    echo -e "  cargo install 実行中..."
    
    if ! command -v cargo >/dev/null 2>&1; then
        echo -e "${RED}[x] エラー: cargo が見つかりません。Rust をインストールしてください: https://rustup.rs/${NC}"
        exit 1
    fi

    SCRIPT_DIR=""
    if [ -n "${BASH_SOURCE[0]}" ] && [ -f "${BASH_SOURCE[0]}" ]; then
        SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    fi

    if [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/Cargo.toml" ]; then
        cd "$SCRIPT_DIR"
        cargo install --path . --force
    else
        cargo install --git "https://github.com/${REPO}.git" --force
    fi
fi

echo ""
echo -e "  ${GREEN}[v] インストール完了: ${INSTALL_DIR}/jpcargo${NC}"

# 4. PATH の確認
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo -e "  ${YELLOW}[!] $INSTALL_DIR が PATH に含まれていません。${NC}"
    echo "  シェル設定（~/.bashrc, ~/.zshrc 等）に追加してください:"
    echo -e "    ${CYAN}export PATH=\"${INSTALL_DIR}:\$PATH\"${NC}"
fi

# 5. 完了案内
echo ""
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD} コマンド例:${NC}"
echo ""
echo -e "  ${CYAN}jpcargo run${NC}                   # cargo run を日本語診断付きで実行"
echo -e "  ${CYAN}jpcargo check${NC}                 # cargo check を日本語診断付きで実行"
echo -e "  ${CYAN}jpcargo list${NC}                  # 対応エラーコード一覧"
echo -e "  ${CYAN}jpcargo explain E0596${NC}         # エラーコード解説"
echo -e "  ${CYAN}jpcargo update${NC}                # 最新版にアップデート"
echo -e "  ${CYAN}jpcargo doctor${NC}                # 開発環境の診断"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
