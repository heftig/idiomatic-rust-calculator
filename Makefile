TARGET := build/calc

RUSTC := rustc
RUSTFLAGS := -C opt-level=z -C overflow-checks=off -C strip=symbols
RUST_TARGET := $(shell $(RUSTC) --print host-tuple)
COMPILE_RUST := $(RUSTC) --edition 2024 --target $(RUST_TARGET) \
		--out-dir build -L dependency=build \
		-C panic=abort -C embed-bitcode=no \
		$(RUSTFLAGS)

RUST_SYSROOT := $(shell $(RUSTC) --print sysroot)
RUSTLIB := $(RUST_SYSROOT)/lib/rustlib/src/rust/library

.PHONY: all dirs clean

all: $(TARGET)

dirs:
	mkdir -p build

clean:
	rm -r build

build/libcore.rmeta: | dirs
	env RUSTC_BOOTSTRAP=1 $(COMPILE_RUST) --crate-name core \
		$(RUSTLIB)/core/src/lib.rs \
		--crate-type lib --emit metadata,link --cap-lints allow \
		--cfg 'feature="optimize_for_size"'

build/libcompiler_builtins.rmeta: build/libcore.rmeta
	env RUSTC_BOOTSTRAP=1 $(COMPILE_RUST) --crate-name compiler_builtins \
		$(RUSTLIB)/compiler-builtins/compiler-builtins/src/lib.rs \
		--crate-type lib --emit metadata,link --cap-lints allow \
		--extern core=$(@D)/libcore.rmeta \
		--cfg 'feature="arch"' \
		--cfg 'feature="compiler-builtins"' \
		--cfg 'feature="default"' \
		--cfg 'feature="unmangled-names"' \
		--cfg 'intrinsics_enabled' \
		--cfg 'optimizations_enabled' \
		--cfg 'f16_enabled' \
		--cfg 'f128_enabled' \
		--cfg 'mem_unaligned'

build/librustc_std_workspace_core.rmeta: build/libcompiler_builtins.rmeta build/libcore.rmeta
	env RUSTC_BOOTSTRAP=1 $(COMPILE_RUST) --crate-name rustc_std_workspace_core \
		$(RUSTLIB)/rustc-std-workspace-core/lib.rs \
		--crate-type lib --emit metadata,link --cap-lints allow \
		--extern compiler_builtins=$(@D)/libcompiler_builtins.rmeta \
		--extern core=$(@D)/libcore.rmeta

build/libpanic_abort.rmeta: build/librustc_std_workspace_core.rmeta
	env RUSTC_BOOTSTRAP=1 $(COMPILE_RUST) --crate-name panic_abort \
		$(RUSTLIB)/panic_abort/src/lib.rs \
		--crate-type lib --emit metadata,link --cap-lints allow \
		--extern core=$(@D)/librustc_std_workspace_core.rmeta

$(TARGET): build/libcore.rmeta build/libcompiler_builtins.rmeta build/libpanic_abort.rmeta \
		main.rs eval.rs
	$(COMPILE_RUST) --crate-name $(@F) \
		main.rs \
		--crate-type bin --emit link \
		--extern compiler_builtins=$(@D)/libcompiler_builtins.rlib \
		--extern core=$(@D)/libcore.rlib \
		--extern panic_abort=$(@D)/libpanic_abort.rlib \
		-lc
