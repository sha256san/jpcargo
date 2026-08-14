#!/usr/bin/env bash
# ==============================================================================
#  jpcargo ワンライナー インストーラースクリプト
#  Rust コンパイラエラー日本語診断ツール (jpcargo) の自動インストール
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

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD} 🦀 jpcargo インストーラー — Rust日本語診断Cargoラッパー${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# 1. Rust / Cargo の存在確認
echo -n "  Checking Cargo / Rust environment ... "
if ! command -v cargo >/dev/null 2>&1; then
    echo -e "${RED}失敗${NC}"
    echo -e "${RED}エラー: cargo がインストールされていません。${NC}"
    echo "先に Rust / Cargo をインストールしてください: https://rustup.rs/"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
echo -e "${GREEN}OK${NC} ($(cargo --version))"

# 2. git の存在確認
if ! command -v git >/dev/null 2>&1; then
    echo -e "${RED}エラー: git がインストールされていません。git をインストールしてください。${NC}"
    exit 1
fi

# 3. インストール元判定（ローカルリポジトリ内か、curl パイプ実行か）
echo ""
echo -e "  ${YELLOW}▶${NC} ${BOLD}jpcargo をビルドして ~/.cargo/bin にインストール中...${NC}"

# スクリプト自身のディレクトリに Cargo.toml があるか判定
SCRIPT_DIR=""
if [ -n "${BASH_SOURCE[0]}" ] && [ -f "${BASH_SOURCE[0]}" ]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
fi

if [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/Cargo.toml" ] && [ -d "$SCRIPT_DIR/src" ]; then
    # ローカルリポジトリから実行
    cd "$SCRIPT_DIR"
    cargo install --path . --force
else
    # curl | bash 等によるリモートパイプ実行
    TMP_DIR="$(mktemp -d)"
    trap 'rm -rf "$TMP_DIR"' EXIT
    git clone --depth 1 https://github.com/sha256san/jpcargo.git "$TMP_DIR"
    cd "$TMP_DIR"
    cargo install --path . --force
fi

echo ""
echo -e "  ${GREEN}✅ インストールが正常に完了しました！${NC}"

# 4. PATH の確認
CARGO_BIN="$HOME/.cargo/bin"
if [[ ":$PATH:" != *":$CARGO_BIN:"* ]]; then
    echo ""
    echo -e "  ${YELLOW}⚠️  警告: $CARGO_BIN が PATH に含まれていません。${NC}"
    echo "  シェル設定ファイル（~/.bashrc, ~/.zshrc 等）に以下を追加してください:"
    echo -e "    ${CYAN}export PATH=\"\$HOME/.cargo/bin:\$PATH\"${NC}"
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
