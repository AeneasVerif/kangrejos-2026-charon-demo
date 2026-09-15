.PHONY: demo1 demo2 demo3 clean

demo1:
	cd ./demo1-kernel && $(MAKE)

demo2:
	cd ./demo2-detect-threads && $(MAKE)

demo3:
	cd ./demo3-transpile && $(MAKE)

clean:
	$(MAKE) -C demo1-kernel clean
	$(MAKE) -C demo2-detect-threads clean
	$(MAKE) -C demo3-transpile clean
