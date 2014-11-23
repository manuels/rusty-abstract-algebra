FILES: \
	libsets.rlib \
	liboperators.rlib \
	libgroups.rlib \
	librings.rlib \
	liblattices.rlib \
	libmodules.rlib	\
	libalgebra.rlib \
	implementations/libquaternions.rlib \
	implementations/test

all: $(FILES)
	#rustc -o prog foo.rlib bar.rlib

lib%.rlib: %.rs
	rustc -L. -L./implementations -o $@ $<

%: %.rs
	rustc -L. -L./implementations -o $@ $<

clean:
	rm -f *.rlib
	rm -f */*.rlib
	rm -f */test
