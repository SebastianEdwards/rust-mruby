.PHONY: all clean examples lib

LIBNAME = libmruby-0.1.rlib
LIB_RS := $(filter-out tests.rs,$(wildcard *.rs))

lib: $(LIBNAME)

all: lib examples

$(LIBNAME): $(LIB_RS)
	rustc -O lib.rs

examples:
	rustc -L. examples/hello.rs

clean:
	rm -f $(LIBNAME) config.rs
	rm -rf doc
	$(MAKE) -C examples clean
