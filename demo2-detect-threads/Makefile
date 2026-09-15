CHARON ?= $(CURDIR)/../bin/charon
.DEFAULT_GOAL := demo

.PHONY: demo clean run-charon
demo: run-charon
	./main.rs use-threads/use_threads.llbc

run-charon:
	cd use-threads && $(CHARON) cargo

clean:
	rm -f detect-threads/*.llbc
