VENV_DIR = $(HOME)/font_venv
VENV_ACTIVATE = $(VENV_DIR)/bin/activate
PYTHON = python3

all: $(VENV_DIR) SyntaxHighlight-Regular.ttf

$(VENV_DIR):
	$(PYTHON) -m venv $(VENV_DIR)
	. $(VENV_ACTIVATE) && pip install fonttools

target/wasm32-unknown-unknown/release/syntax_highlight.wasm: src/lib.rs Cargo.toml
	cargo build --target wasm32-unknown-unknown --release

SyntaxHighlight-Regular.ttf: target/wasm32-unknown-unknown/release/syntax_highlight.wasm
	cp UbuntuSansMono.ttf SyntaxHighlight-Regular.ttf
	wasm-strip target/wasm32-unknown-unknown/release/syntax_highlight.wasm
	. $(VENV_ACTIVATE) && ../bin/otfsurgeon -i SyntaxHighlight-Regular.ttf add wasm < target/wasm32-unknown-unknown/release/syntax_highlight.wasm
	mv SyntaxHighlight-Regular-add.ttf SyntaxHighlight-Regular.ttf

.PHONY: clean
clean:
	rm -f SyntaxHighlight-Regular.ttf
	cargo clean