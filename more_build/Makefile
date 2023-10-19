M ?= pb

ifeq ($(M),ta6nt)
LZ4 := lz4.dll
ZLIB := zlib1.dll
KERNEL := scheme.dll
SCHEME := scheme.exe
else
LZ4 := liblz4.so
ZLIB := libz.so
KERNEL := libscheme.so
SCHEME := scheme
endif

DIST_C := $(LZ4) $(ZLIB) $(KERNEL) $(SCHEME)
DIST_S := equates.h scheme.h petite.boot scheme.boot

.PHONY: all boot kernel clean

all: dist

ifeq ($(DB),1)
export CFLAGS += -MJ $(addsuffix .json,$$(@))
db: kernel
	echo '' > compile_commands.json
	echo '[' >> compile_commands.json
	fd json build/c.$(M) -x cat {} >> compile_commands.json
	echo ']' >> compile_commands.json
endif

kernel: $(addprefix build/boot/$(M)/, $(DIST_S))
	$(MAKE) -f kernel.mk M=$(M)

boot:
	$(MAKE) -f boot.mk M=$(M)

clean:
	$(RM) -r build

dist: $(addprefix build/dist.$(M)/, $(DIST_C) $(DIST_S))

build/dist.$(M)/:
	mkdir -p $@

$(addprefix build/c.$(M)/, $(DIST_C)): kernel

$(addprefix build/boot/$(M)/, $(DIST_S)): boot

$(addprefix build/dist.$(M)/, $(DIST_C)): build/dist.$(M)/%: build/c.$(M)/% | build/dist.$(M)/
	cp $< $@

$(addprefix build/dist.$(M)/, $(DIST_S)): build/dist.$(M)/%: build/boot/$(M)/% | build/dist.$(M)/
	cp $< $@
