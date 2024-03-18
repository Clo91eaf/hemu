ELF_FILES := $(shell find elf -type f)
BIN_FILES := $(patsubst elf/%, bin/%, $(ELF_FILES))

all: $(BIN_FILES)

bin/%: elf/%
	riscv64-unknown-elf-objcopy -O binary $< $@

clean:
	rm -f $(BIN_FILES)