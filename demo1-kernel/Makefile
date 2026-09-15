LINUX ?= ../linux
CHARON_BIN ?= charon
CHARON_OPTIONS ?=

LINUX := $(abspath $(LINUX))
CHARON_RUSTC ?= $(shell $(CHARON_BIN) toolchain-path)/bin/rustc
CHARON_OUTPUT := $(CURDIR)/kernel.llbc
CHARON_PRETTY := $(CURDIR)/kernel.pretty

BUILD := $(CURDIR)/build
WRAPPER := $(CURDIR)/rustc-wrapper
KBUILD := $(MAKE) -C $(LINUX) O=$(BUILD) LLVM=1 HOSTCC=gcc HOSTCXX=g++ RUSTC=$(WRAPPER)

export CHARON_BIN CHARON_OPTIONS CHARON_OUTPUT CHARON_RUSTC

.DEFAULT_GOAL := demo

.PHONY: demo kernel.llbc check clean
demo: kernel.pretty
	grep -A24 'Full name: bindings::bindings_raw::list_head$$' $<

kernel.llbc: build/.config rustc-wrapper
	rm -f $(BUILD)/rust/kernel.o $(BUILD)/rust/libkernel.rmeta $(BUILD)/rust/.kernel.o.cmd
	$(KBUILD) rust/kernel.o

kernel.pretty: kernel.llbc
	$(CHARON_BIN) pretty-print --include-layouts --hide-storage-statements $< > $@

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
	rm -f $(CHARON_OUTPUT) $(CHARON_PRETTY)
