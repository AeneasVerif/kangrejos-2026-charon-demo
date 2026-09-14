CHARON ?= ../bin/charon
.DEFAULT_GOAL := demo

.PHONY: demo clean
demo: input.llbc
	./main.rs $<

input.llbc: input.rs
	$(CHARON) rustc --format=json --dest-file=$@ -- $< --crate-type=rlib --edition=2021

clean:
	rm -f input.llbc
