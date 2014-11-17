FILES: \
	libsets.rlib \
	liboperators.rlib \
	libgroups.rlib \
	librings.rlib \
	liblattices.rlib \
	libmodules.rlib

all: $(FILES)
	#rustc -o prog foo.rlib bar.rlib

lib%.rlib: %.rs
	rustc -L. -o $@ $<
