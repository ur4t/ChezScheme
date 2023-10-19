# the following flags control various compiler options.  flags prefixed by an x
# separately control the options used while compiling a cross compiler.

# o determines the optimize level
o = 3

# d is the debug level at which the system should be built
d = 0

# cl determines the commonization level
cl = (commonization-level)

# i determines whether inspector-information is generated: f for false, t for true
i = f

# cp0 determines the number of cp0 (source optimizer) iterations run
cp0 = 2

# fc determines whether fasl objects are compressed
fc = t

# xf determines the compression format
xf = (compress-format)

# xl determine the compression level
xl = (compress-level)

# p (xp) determines whether source profiling is enabled: f for false, t for true.
p = f
xp = f

# bp (xpb) determines whether binary profiling is enabled: f for false, t for true.
bp = f
xbp = f

# loadspd determines whether source-profile data is loaded: f for false, t for true
loadspd = f

# dumpspd determines whether source-profile data is dumped: f for false, t for true
dumpspd = f

# loadbpd determines whether binary-profile data is loaded: f for false, t for true
loadbpd = f

# dumpbpd determines whether binary-profile data is dumped: f for false, t for true
dumpbpd = f

# compile determines the entry point for compilng files
# another useful value for this is compile-with-asm, defined in debug.ss
compile = compile-file

# pdhtml determines whether profile-dump-html is called at the end of a build
pdhtml = f

# gac determines whether cost-center allocation counts are generated: f for false, t for true
gac = f

# gic determines whether cost-center instruction counts are generated: f for false, t for true
gic = f

# pps determines whether pass timings are printed
pps = f

# TIME determines whether time used to compile objects are printed
# begin can be used to replace time
# TIME = time
TIME = begin

O ?= build/s.xc.$(M)/

ifeq ($(V),1)
Q =
else
Q = @
MAKEFLAGS += --no-print-directory
endif

MK_PATH := $(dir $(lastword $(MAKEFILE_LIST)))

vpath %.ss $(patsubst %, $(MK_PATH)../%/, c s nanopass unicode)
vpath %.so $(patsubst %, $(O)%/, macro mkinc nanopass patch)
vpath machine.def $(O)

CHEZ_S_PATH := $(MK_PATH)../s/
NANOPASS_PATH := $(MK_PATH)../nanopass/
UNICODE_PATH := $(MK_PATH)../unicode/

SCHEME := scheme

ProfileDumpSource = $(O)source.pd
ProfileDumpBlock  = $(O)block.pd


# sources required to build cross compiler
# putting cpnanopass.patch early for maximum make --jobs=2 benefit
PATCH_OBJS := patch cpnanopass cpprim cprep cpcheck\
 cp0 cpvalid cptypes cpcommonize cpletrec reloc\
 compile fasl vfasl pbchunk syntax env\
 read interpret ftype strip ubify back
PATCH_OBJS := $(patsubst %, $(O)patch/%.so, $(PATCH_OBJS))

# ordering constraints in "petite.boot":
#  first: library, prims, mathprims, front, 5_?
#  last: back
#  newhash before read
#  io before read
#  event before 4
#  ftype after syntax
#  layout and record before strnum (first define-record)
#  date before 7
# (there may be other constraints as well)

BASE_OBJS := library prims mathprims record 5_1 5_2 5_3\
 strnum bytevector 5_4 5_6 5_7 event 4 front foreign 6 print newhash\
 format date 7 cafe trace engine interpret cprep cpcheck cp0 cpvalid cptypes\
 cpcommonize cpletrec inspect enum io read primvars syntax costctr expeditor\
 exceptions pretty env fasl vfasl pbchunk reloc pdhtml strip ftype back
BASE_OBJS := $(patsubst %, $(O)boot/%.$(M), $(BASE_OBJS))

COMPILER_OBJS := cpnanopass cpprim compile cback
COMPILER_OBJS := $(patsubst %, $(O)boot/%.$(M), $(COMPILER_OBJS))

MACRO_OBJS := cmacros priminfo primvars env setup
MACRO_OBJS := $(patsubst %, $(O)macro/%.so, $(MACRO_OBJS))

MORE_SRCS := base-lang.ss expand-lang.ss primref.ss\
 types.ss io-types.ss fasl-helpers.ss hashtable-types.ss

MKINC_OBJS := mkgc mkheader
MKINC_OBJS := $(patsubst %, $(O)mkinc/%.so, $(MKINC_OBJS))

GC_INCLUDES := gc-ocd gc-oce gc-par heapcheck
GC_INCLUDES := $(patsubst %, $(O)../boot/$(M)/%.inc, $(GC_INCLUDES))

PETITE_BOOT := $(O)../boot/$(M)/petite.boot
SCHEME_BOOT := $(O)../boot/$(M)/scheme.boot
SCHEME_HDR  := $(O)../boot/$(M)/scheme.h
EQUATES_HDR := $(O)../boot/$(M)/equates.h

all: $(PETITE_BOOT) $(SCHEME_BOOT) $(SCHEME_HDR) $(EQUATES_HDR) $(GC_INCLUDES)

clean:
	$(RM) -r $(addprefix $(O), ../boot/$(M) macro mkinc nanopass patch boot)

$(O) $(patsubst %, $(O)%/, ../boot/$(M) macro mkinc nanopass patch boot):
	$(Q)mkdir -p $@

# common handlers, some levels
define SETUP_BASIC
'(reset-handler abort)'\
'(base-exception-handler (lambda (c) (fresh-line) (display-condition c) (newline) (reset)))'\
'(keyboard-interrupt-handler (lambda () (display "interrupted---aborting\n") (reset)))'\
'(optimize-level $o)'\
'(debug-level $d)'\
'(commonization-level $(cl))'\
'(fasl-compressed #$(fc))'\
'(compress-format $(xf))'\
'(compress-level $(xl))'\
'(generate-inspector-information #$i)'\
'(generate-allocation-counts #${gac})'\
'(generate-instruction-counts #${gic})'\
''
endef

define SETUP_GC
'(collect-trip-bytes (expt 2 24))'\
'(collect-request-handler (lambda () (collect 0 1)))'\
'(collect 1 2)'\
''
endef

define SETUP_CP0
'(run-cp0 (lambda (cp0 x) (do ([i ${cp0} (fx- i 1)] [x x (cp0 x)]) ((fx= i 0) x))))'
endef

# dump profile
define DUMP_PROFILE
'(when #${pdhtml} (profile-dump-html))'\
'(when #${dumpspd} (profile-dump-data "${ProfileDumpSource}"))'\
'(when #${dumpbpd} (profile-dump-data "${ProfileDumpBlock}"))'\
''
endef

define SETUP_NANOPASS_PATH
'(library-directories (list (cons "$(NANOPASS_PATH)" "$(O)nanopass")))'
endef

ifeq ($(M),ta6nt)
MACHINE_DEF := '(features iconv expeditor pthreads windows)(include "a6.def")(include "nt.def")'
MACHINE_INCLUDE := x86_64.ss
endif
ifeq ($(M),ta6le)
MACHINE_DEF := '(features iconv expeditor pthreads)(define-constant time-t-bits 64)(include "a6.def")'
MACHINE_INCLUDE := x86_64.ss
endif
ifeq ($(M),pb)
MACHINE_DEF := '(features)(include "pbcommon64.def")(include "pbcommon.def")'
MACHINE_INCLUDE := pb.ss
endif

$(MACRO_OBJS): | $(O)macro/

$(O)macro/cmacros.so: layout.ss machine.def

$(O)macro/priminfo.so: primdata.ss cmacros.so

$(O)macro/primvars.so: primref.ss

$(O)macro/setup.so: debug.ss

$(O)macro/primvars.so $(O)macro/setup.so $(O)macro/env.so: cmacros.so priminfo.so

$(O)mkinc/mkgc.so: mkheader.so

$(MKINC_OBJS): $(filter-out $(O)macro/setup.so, $(MACRO_OBJS)) | $(O)mkinc/

$(O)machine.def: | $(O)
	$(Q)echo '(define-constant machine-type (constant machine-type-$(M)))' $(MACHINE_DEF) '(include "default.def")' > $@

# "$(O)" for machine.def, needed by cmacros.so
$(O)mkinc/%.so $(O)macro/%.so: %.ss
	$(Q)echo $(SETUP_BASIC) '(subset-mode (quote system))'\
	'(source-directories (list "$(O)" "$(CHEZ_S_PATH)"))'\
	'(compile-file "$<" "$@" (machine-type))'\
	| $(SCHEME) -q $(filter %.so, $^)

$(O)nanopass/nanopass.so: nanopass.ss $(wildcard $(NANOPASS_PATH)nanopass/*.ss) | $(O)nanopass/
	$(Q)echo $(SETUP_BASIC) $(SETUP_GC) $(SETUP_NANOPASS_PATH)\
	'(compile-library "$<" "$@" (machine-type))'\
	| $(SCHEME) -q $(filter %.so, $^) --compile-imported-libraries

$(O)boot/5_4.$(M): unicode-char-cases.ss unicode-charinfo.ss

$(O)boot/cpnanopass.$(M) $(O)patch/cpnanopass.so: nanopass.so np-languages.ss $(MACHINE_INCLUDE)

$(BASE_OBJS) $(COMPILER_OBJS): $(O)boot/%.$(M): %.ss $(MACRO_OBJS) $(PATCH_OBJS) $(MORE_SRCS) | $(O)boot/
	$(Q)echo $(SETUP_BASIC)\
	'(when #$p (compile-profile (quote source)))'\
	'(when #$(bp) (compile-profile (quote block)))'\
	'(when #$(loadspd) (profile-load-data "${ProfileDumpSource}"))'\
	'(when #$(loadbpd) (profile-load-data "${ProfileDumpBlock}"))'\
	$(SETUP_CP0)\
	$(SETUP_GC)\
	'(print-gensym (quote pretty/suffix))'\
	'(source-directories (list "$(CHEZ_S_PATH)" "$(UNICODE_PATH)"))'\
	$(SETUP_NANOPASS_PATH)\
	'($(TIME) (${compile} "$<" "$@" (quote $(M))))'\
	$(DUMP_PROFILE)\
	| $(SCHEME) -q $(filter %.so, $^)

$(PATCH_OBJS): $(O)patch/%.so: %.ss $(MACRO_OBJS) nanopass.so $(MORE_SRCS) | $(O)patch/
	$(Q)echo $(SETUP_BASIC)\
	'(when #$(xp) (compile-profile (quote source)))'\
	'(when #$(xbp) (compile-profile (quote block)))'\
	$(SETUP_CP0)\
	$(SETUP_GC)\
	'(print-gensym (quote pretty/suffix))'\
	'(source-directories (list "$(CHEZ_S_PATH)"))'\
	$(SETUP_NANOPASS_PATH)\
	'($(TIME) (${compile} "$<" "$@" (machine-type)))'\
	$(DUMP_PROFILE)\
	| $(SCHEME) -q $(filter %.so, $^)

$(SCHEME_HDR) $(EQUATES_HDR) $(PETITE_BOOT) $(SCHEME_BOOT): | $(O)../boot/$(M)/

$(PETITE_BOOT): $(BASE_OBJS)

$(SCHEME_BOOT): $(COMPILER_OBJS)

$(SCHEME_BOOT): BASE_BOOT := "petite"

$(PETITE_BOOT) $(SCHEME_BOOT): $(MACRO_OBJS) $(PATCH_OBJS)
	$(Q)echo '(reset-handler abort)'\
	'(apply #%$$make-boot-file "$@" (quote $(M)) (quote ($(BASE_BOOT)))'\
	'  (map symbol->string (quote ($(filter %.$(M), $^)))))'\
	| $(SCHEME) -q $(filter %.so, $^)

$(SCHEME_HDR): MORE_ARGS := (quote $(M))

$(GC_INCLUDES): mkgc.so

$(GC_INCLUDES) $(SCHEME_HDR) $(EQUATES_HDR): nanopass.so $(MACRO_OBJS) $(MORE_SRCS) mkheader.so
	$(Q)echo '(reset-handler abort)'\
	'(mk$(@F) "$@" $(MORE_ARGS))'\
	| $(SCHEME) -q $(filter %.so, $^)
