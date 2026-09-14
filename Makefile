CHARON_DIR ?= "$(HOME)/wip/work/charon"
CHARON_BIN ?= "$(PWD)/bin/charon"

demo1:
	cd ./demo1-kernel && $(MAKE)

demo2:
	cd ./demo2-detect-threads && $(MAKE)

demo3:
	cd ./demo3-transpile && $(MAKE)
