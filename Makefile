TARGET := build/calc

.PHONY: all dirs clean

all: $(TARGET)

dirs:
	mkdir -p build

clean:
	rm -r build

build/main.o: main.rs | dirs
	rustc main.rs --emit=obj -o build/main.o -C panic=abort

$(TARGET): build/main.o | dirs 
	clang build/main.o -o $(TARGET)
