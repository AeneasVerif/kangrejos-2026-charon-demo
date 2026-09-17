.PHONY: all demo1 demo2 demo3 demo4 clean

all: demo1 demo2 demo3 demo4

demo1:
	cd ./demo1-kernel && $(MAKE)

demo2:
	cd ./demo2-detect-threads && $(MAKE)

demo3:
	cd ./demo3-transpile && $(MAKE)

demo4:
	cd ./demo4-editions && $(MAKE)

clean:
	$(MAKE) -C demo1-kernel clean
	$(MAKE) -C demo2-detect-threads clean
	$(MAKE) -C demo3-transpile clean
	$(MAKE) -C demo4-editions clean
