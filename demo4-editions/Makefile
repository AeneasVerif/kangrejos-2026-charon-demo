LINUX ?= ../linux
CHARON_BIN ?= charon
CHARON_OPTIONS ?=

LINUX := $(abspath $(LINUX))
CHARON_RUSTC ?= $(shell $(CHARON_BIN) toolchain-path)/bin/rustc
CHARON_OUTPUT_2021 := $(CURDIR)/kernel-2021.llbc.postcard
CHARON_OUTPUT_2024 := $(CURDIR)/kernel-2024.llbc.postcard

BUILD := $(CURDIR)/build
WRAPPER := $(CURDIR)/rustc-wrapper
KBUILD := $(MAKE) -C $(LINUX) O=$(BUILD) LLVM=1 HOSTCC=gcc HOSTCXX=g++ RUSTC=$(WRAPPER)

export CHARON_BIN CHARON_OPTIONS CHARON_OUTPUT_2021 CHARON_OUTPUT_2024 CHARON_RUSTC

.DEFAULT_GOAL := demo

.PHONY: demo check clean
demo: kernel-2021.llbc.postcard kernel-2024.llbc.postcard
	./main.rs $^

kernel-2021.llbc.postcard kernel-2024.llbc.postcard &: build/.config rustc-wrapper
	rm -f $(BUILD)/rust/kernel.o
	$(KBUILD) rust/kernel.o

build/.config:
	mkdir -p $(BUILD)
	$(KBUILD) defconfig
	$(LINUX)/scripts/config --file $@ --enable RUST
	$(KBUILD) olddefconfig
	grep -qx 'CONFIG_RUST=y' $@

check:
	$(KBUILD) rustavailable

clean:
	$(KBUILD) clean
	rm -rf $(BUILD)
	rm -f $(CHARON_OUTPUT_2021) $(CHARON_OUTPUT_2024)
