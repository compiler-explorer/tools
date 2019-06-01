GHC:=../.travis-compilers/ghc/bin/ghc
demangle: demangle.hs
	$(GHC) -package ghc -dynamic demangle.hs -optl-Wl,-rpath,'$$ORIGIN' 

clean:
	rm -f demangle.o demangle demangle.hi *.so*
