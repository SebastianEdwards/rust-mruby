.PHONY: all clean examples lib

LIBNAME = libmruby-0.1.rlib
LIB_RS := $(filter-out tests.rs,$(wildcard *.rs))

lib: $(LIBNAME)

all: lib examples

$(LIBNAME): $(LIB_RS)
	rustc -O lib.rs

examples:
	rustc -L. examples/hello.rs
	rustc -L. examples/call_rust_fn.rs

clean:
	rm -f $(LIBNAME) config.rs
	rm -rf doc
	rm examples/hello
	rm examples/call_rust_fn
