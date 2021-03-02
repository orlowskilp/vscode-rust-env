CC=cargo
PROJ=find_replace
BIN_DIR=bin

.PHONY: all
all:
	$(CC) build

.PHONY: release
release:
	$(CC) build --release
	mkdir bin
	cp target/release/$(PROJ) $(BIN_DIR)/$(PROJ)

.PHONY: clean
test:
	$(CC) test

.PHONY: clean
clean:
	rm -rf bin
	$(CC) clean 
