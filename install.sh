#!/usr/bin/env bash
# ==============================================================================
#  jpcargo 高速インストーラー（事前ビルド済みバイナリ直接ダウンロード）
#  GitHub Actions でコンパイルされたバイナリを直接ダウンロードして即座に配置します
#
#  使い方:
#    curl -fsSL https://raw.githubusercontent.com/sha256san/jpcargo/main/install.sh | bash
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
echo -e "${BOLD} 🦀 jpcargo 高速インストーラー（GitHub Actions 事前ビルドバイナリ）${NC}"
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

# 2. 事前ビルド済みバイナリのダウンロード試行 (GitHub Releases)
if [ -n "$OS_TARGET" ] && [ -n "$ARCH_TARGET" ]; then
    TARGET="${ARCH_TARGET}-${OS_TARGET}"
    ARCHIVE_NAME="jpcargo-${TARGET}.tar.gz"
    DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ARCHIVE_NAME}"

    echo -e "  検出環境: ${BOLD}${TARGET}${NC}"
    echo -n "  ▶ GitHub Releases からバイナリをダウンロード中... "

    TMP_DIR="$(mktemp -d)"
    trap 'rm -rf "$TMP_DIR"' EXIT

    # ダウンロード試行（HTTPステータスを確認してエラー時はフォールバック）
    if curl -fL -sS "$DOWNLOAD_URL" -o "$TMP_DIR/$ARCHIVE_NAME" 2>/dev/null; then
        if tar -xzf "$TMP_DIR/$ARCHIVE_NAME" -C "$TMP_DIR" 2>/dev/null; then
            if [ -f "$TMP_DIR/jpcargo" ]; then
                mv "$TMP_DIR/jpcargo" "$INSTALL_DIR/jpcargo"
                chmod +x "$INSTALL_DIR/jpcargo"
                echo -e "${GREEN}完了 ⚡ (コンパイル不要で即座にインストール)${NC}"
                INSTALLED=true
            fi
        fi
    fi

    if [ "$INSTALLED" = false ]; then
        echo -e "${YELLOW}スキップ (Releaseバイナリが見つからないためローカルビルドを実行します)${NC}"
    fi
fi

# 3. 事前ビルドが取得できなかった場合のフォールバック (cargo install)
if [ "$INSTALLED" = false ]; then
    echo ""
    echo -e "  ${YELLOW}▶${NC} ${BOLD}ソースコードから cargo install を実行中...${NC}"
    
    if ! command -v cargo >/dev/null 2>&1; then
        echo -e "${RED}エラー: cargo が見つかりません。Rust をインストールしてください: https://rustup.rs/${NC}"
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
echo -e "  ${GREEN}✅ インストールが正常に完了しました！ -> ${INSTALL_DIR}/jpcargo${NC}"

# 4. PATH の確認
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo -e "  ${YELLOW}⚠️  警告: $INSTALL_DIR が PATH に含まれていません。${NC}"
    echo "  シェル設定ファイル（~/.bashrc, ~/.zshrc 等）に以下を追加してください:"
    echo -e "    ${CYAN}export PATH=\"${INSTALL_DIR}:\$PATH\"${NC}"
fi

# 5. 完了案内
echo ""
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD} 🎉 準備完了！ 以下のコマンドを試してみましょう:${NC}"
echo ""
echo -e "  ${CYAN}jpcargo doctor${NC}                # 開発環境の総合診断"
echo -e "  ${CYAN}jpcargo run${NC}                   # cargo run を日本語エラー診断付きで実行"
echo -e "  ${CYAN}jpcargo check${NC}                 # cargo check を日本語エラー診断付きで実行"
echo -e "  ${CYAN}jpcargo list${NC}                  # 対応している全518種類のエラーコード一覧"
echo -e "  ${CYAN}jpcargo explain E0596${NC}         # 指定エラーコードの日本語解説を表示"
echo -e "  ${CYAN}jpcargo --level beginner check${NC}# 初心者向け解説モードで実行"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
