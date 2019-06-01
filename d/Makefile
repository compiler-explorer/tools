GDC:=../.travis-compilers/gdc/x86_64-pc-linux-gnu/bin/gdc
demangle: demangle.d
	$(GDC) -O2 $< -o $@

clean:
	rm -f demangle.o demangle
