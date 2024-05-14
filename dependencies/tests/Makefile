ELF_FILES := $(shell find elf -type f)
ASM_FILES := $(patsubst elf/%, asm/%, $(ELF_FILES))
BIN_FILES := $(patsubst elf/%, bin/%, $(ELF_FILES))

all: $(BIN_FILES) $(ASM_FILES)

bin/%: elf/%
	riscv64-unknown-elf-objcopy -O binary $< $@

asm/%: elf/%
	riscv64-linux-gnu-objdump -d -M no-aliases,numeric $< > $@

clean:
	rm -f $(BIN_FILES) $(ASM_FILES)