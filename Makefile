TARGET := build/calc

.PHONY: all dirs clean

all: $(TARGET)

dirs:
	mkdir -p build

clean:
	rm -r build

build/main.o: main.rs | dirs
	rustc main.rs --emit=obj -o build/main.o -C panic=abort -C overflow-checks=off -C opt-level=z

build/eval.o: eval.rs | dirs
	rustc eval.rs --emit=obj -o build/eval.o -C panic=abort -C overflow-checks=off -C opt-level=z

$(TARGET): build/main.o build/eval.o | dirs 
	clang build/main.o build/eval.o -o $(TARGET)
