CC = riscv64-unknown-elf-gcc
OBJCOPY = riscv64-unknown-elf-objcopy
OBJDUMP = riscv64-unknown-elf-objdump

SRC_DIR = ./src
BUILD_DIR = ./build

C_FILES = $(wildcard $(SRC_DIR)/*.c)
S_FILES = $(patsubst $(SRC_DIR)/%.c,$(BUILD_DIR)/%.s,$(C_FILES))
ELF_FILES = $(patsubst $(SRC_DIR)/%.c,$(BUILD_DIR)/%.elf,$(C_FILES))
BIN_FILES = $(patsubst $(SRC_DIR)/%.c,$(BUILD_DIR)/%.bin,$(C_FILES))
D_FILES = $(patsubst $(SRC_DIR)/%.c,$(BUILD_DIR)/%.d,$(C_FILES))

.PHONY: all clean

# all: $(BIN_FILES)
all:  $(BIN_FILES) $(D_FILES)

$(shell mkdir -p $(BUILD_DIR))

$(BUILD_DIR)/%.s: $(SRC_DIR)/%.c
	$(CC) -S -nostdlib -march=rv64i2p0_m2p0_a2p0_f2p0_d2p0 $< -o $@

$(BUILD_DIR)/%.elf: $(BUILD_DIR)/%.s
	$(CC) -Wl,-Ttext=0x80000000 -nostdlib -o $@ $<

$(BUILD_DIR)/%.bin: $(BUILD_DIR)/%.elf
	$(OBJCOPY) -O binary $< $@

$(BUILD_DIR)/%.d: $(BUILD_DIR)/%.elf
	$(OBJDUMP) -d $< > $@

clean:
	rm -rf $(BUILD_DIR)
