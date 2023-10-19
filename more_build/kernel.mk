O ?= build/c.$(M)/

MK_PATH := $(dir $(lastword $(MAKEFILE_LIST)))

CFLAGS += -pthread -O2 -m64 -msse2 -Wextra -Wall -Wno-unused-parameter
LDFLAGS += -L $(O) -fuse-ld=lld

ifeq ($(M),ta6nt)
CC := clang --target=x86_64-windows-msvc
CFLAGS += -D _CRT_SECURE_NO_WARNINGS -D _CRT_NONSTDC_NO_DEPRECATE
LDFLAGS += -l msvcrt
else
CFLAGS += -fPIC
LDFLAGS += -z now -z origin -z noexecstack
endif

ifeq ($(V),1)
Q =
msg =
else
Q = @
msg = @printf '  %-8s %s%s\n' "$(1)" "$(2)" "$(if $(3), $(3))";
MAKEFLAGS += --no-print-directory
endif

define ARCHIVE
$(call msg,AR,$@)
$(Q)$(AR) rcs $@ $(filter %.o, $^)
endef

define DYNAMIC
$(call msg,CCLD,$@)
$(Q)$(CC) -o $@ $(LDFLAGS) $(filter %.o, $^) -shared
endef

define LIBRARY
$(if $(filter .a,$(suffix $@)),$(ARCHIVE),$(DYNAMIC))
endef

.PHONY: all
all: build

LZ4_OBJS := lz4 lz4frame lz4hc xxhash
ZLIB_OBJS := adler32 crc32 deflate\
 infback inffast inflate inftrees trees zutil\
 compress uncompr gzclose gzlib gzread gzwrite
KERNEL_OBJS := statics segment alloc symbol intern\
 gcwrapper gc-011 gc-par gc-ocd gc-oce\
 number schsig io new-io print fasl vfasl\
 stats foreign prim prim5 flushcache schlib thread\
 expeditor scheme compress-io random ffi self-exe
MAIN_OBJS := main

ifeq ($(M),ta6nt)

KERNEL_OBJS += windows
LZ4 := lz4.dll
ZLIB := zlib1.dll

KERNEL_CFLAGS += -D X86_64
KERNEL_LDFLAGS += -l lz4 -l zlib1
KERNEL_LDFLAGS += -l advapi32 -l user32 -l ole32 -l rpcrt4

ASM_OBJ := $(O)kernel/a6nt-jump.o
$(O)kernel/a6nt-jump.o: $(MK_PATH)a6nt-jump.S

KERNEL := scheme.dll
SCHEME := scheme.exe

LZ4_CFLAGS := -D LZ4_DLL_EXPORT=1
ZLIB_CFLAGS := -D ZLIB_DLL

else # ifeq ($(M),ta6nt)

ifeq ($(M),ta6le)
KERNEL_OBJS += i3le
KERNEL_CFLAGS += -D X86_64
endif

ifeq ($(M),pb)
KERNEL_OBJS += pb
KERNEL_CFLAGS += -D PORTABLE_BYTECODE
endif

LZ4 := liblz4.so
ZLIB := libz.so
ZLIB_CFLAGS := -D HAVE_UNISTD_H
KERNEL_LDFLAGS += -l lz4 -l z
KERNEL_LDFLAGS += -l m -l pthread

KERNEL := libscheme.so
SCHEME := scheme
MAIN_LDFLAGS += -Wl,-rpath='$$ORIGIN'
MAIN_LDFLAGS += -l curses

endif # ifeq ($(M),ta6nt)

KERNEL_CFLAGS += -I $(MK_PATH) -I $(O)../boot/$(M)/
KERNEL_CFLAGS += $(patsubst %, -I $(MK_PATH)../%/, lz4/lib zlib)
KERNEL_CFLAGS += -D _REENTRANT

LZ4_OBJS := $(patsubst %, $(O)lz4/%.o, $(LZ4_OBJS))
ZLIB_OBJS := $(patsubst %, $(O)zlib/%.o, $(ZLIB_OBJS))
KERNEL_OBJS := $(patsubst %, $(O)kernel/%.o, $(KERNEL_OBJS))
MAIN_OBJS := $(patsubst %, $(O)main/%.o, $(MAIN_OBJS))

.PHONY: build
build: $(O)$(SCHEME)

.PHONY: clean
clean:
	$(RM) -r $(patsubst %, $(O)%/, lz4 zlib kernel main)

$(patsubst %, $(O)%/, lz4 zlib kernel main):
	$(call msg,MKDIR,$@)
	$(Q)mkdir -p $@

$(LZ4_OBJS): CFLAGS += $(LZ4_CFLAGS)
$(ZLIB_OBJS): CFLAGS += -D HAVE_STDARG_H $(ZLIB_CFLAGS)
$(KERNEL_OBJS): CFLAGS += $(KERNEL_CFLAGS)
$(MAIN_OBJS): CFLAGS += -I $(MK_PATH) -I $(O)../boot/$(M)/

$(LZ4_OBJS): $(O)lz4/%.o: $(MK_PATH)../lz4/lib/%.c $(O)lz4/%.o.d | $(O)lz4/
$(ZLIB_OBJS): $(O)zlib/%.o: $(MK_PATH)../zlib/%.c $(O)zlib/%.o.d | $(O)zlib/
$(KERNEL_OBJS): $(O)kernel/%.o: $(MK_PATH)../c/%.c $(O)kernel/%.o.d | $(O)kernel/
$(MAIN_OBJS): $(O)main/%.o: $(MK_PATH)../c/%.c $(O)main/%.o.d | $(O)main/

$(LZ4_OBJS) $(ZLIB_OBJS) $(KERNEL_OBJS) $(MAIN_OBJS) $(ASM_OBJ):
	$(call msg,CC,$@)
	$(Q)$(CC) -o $@ -MT $@ -MMD -MP -MF $@.d $(CFLAGS) $< -c

$(LZ4_OBJS:.o=.o.d):
$(ZLIB_OBJS:.o=.o.d):
$(KERNEL_OBJS:.o=.o.d):
$(MAIN_OBJS:.o=.o.d):

# modified variables will be passed to prerequisites
$(O)$(LZ4) $(O)$(ZLIB): LDFLAGS := $(LDFLAGS)
$(O)$(KERNEL): LDFLAGS += $(KERNEL_LDFLAGS)
$(O)$(SCHEME): LDFLAGS += $(MAIN_LDFLAGS)

$(O)$(LZ4): $(LZ4_OBJS)
$(O)$(ZLIB): $(ZLIB_OBJS)
$(O)$(KERNEL): $(KERNEL_OBJS) $(ASM_OBJ) $(O)$(LZ4) $(O)$(ZLIB)

$(O)$(LZ4) $(O)$(ZLIB) $(O)$(KERNEL):
	$(LIBRARY)

$(O)$(SCHEME): $(MAIN_OBJS) $(O)$(KERNEL)
	$(call msg,CCLD,$@)
	$(Q)$(CC) -o $@ $(LDFLAGS) $(filter %.o, $^) -l scheme

include $(wildcard $(O)lz4/*.d)
include $(wildcard $(O)zlib/*.d)
include $(wildcard $(O)kernel/*.d)
include $(wildcard $(O)main/*.d)
