use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, short = 'q', help = "Suppress greeting and prompt")]
    quiet: bool,
    #[arg(long, help = "Run as shell script")]
    script: Option<PathBuf>,
    #[arg(long, help = "Run rnrs program as shell script")]
    program: Option<PathBuf>,
    #[arg(long, help = "Set library directories")]
    libdirs: Option<String>,
    #[arg(long, help = "Set library extensions")]
    libexts: Option<String>,
    #[arg(long, help = "Compile libraries before loading")]
    compile_imported_libraries: bool,
    #[arg(long, help = "Disable libraries timestamps")]
    disable_library_timestamps: bool,
    #[arg(long, help = "Enable import search messages")]
    import_notify: bool,
    #[arg(long, help = "Set optimize-level", default_value = "0")]
    optimize_level: isize,
    #[arg(long, help = "On uncaught exception, call debug")]
    debug_on_exception: bool,
    #[arg(long, help = "Disable expression editor")]
    eedisable: bool,
    #[arg(long, help = "Expression-editor history file")]
    eehistory: Option<PathBuf>,
    #[arg(long, help = "Have collector maintain object counts")]
    enable_object_counts: bool,
    #[arg(long, help = "Keep reloc info for compute-size, etc.")]
    retain_static_relocation: bool,
    #[arg(long, short = 'b', help = "Load boot file")]
    boot: Option<PathBuf>,
    #[arg(long, help = "Trace boot/heap search process")]
    verbose: bool,
    #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
    trailing: Vec<String>,
}

fn main() {
    use kernel::*;
    let exec_path = std::env::current_exe().unwrap();
    let exec_path = exec_path.file_name().unwrap().to_str().unwrap();
    let args = Args::parse();

    scheme_init();

    if let Some(ref boot) = args.boot {
        register_boot_executable_relative_file(exec_path, boot.to_str().unwrap());
    }

    if args.verbose {
        set_verbose();
    }

    build_heap(exec_path);

    if args.quiet {
        call1("suppress-greeting", r#true());
        call1("waiter-prompt-string", string(""));
    }

    if args.retain_static_relocation {
        retain_static_relocation();
    }

    if args.enable_object_counts {
        call1("enable-object-counts", r#true());
    }

    if args.optimize_level != 0 {
        call1("optimize-level", integer(args.optimize_level));
    }

    if args.debug_on_exception {
        call1("debug-on-exception", r#true());
    }

    if args.import_notify {
        call1("import-notify", r#true());
    }

    if let Some(ref libdirs) = args.libdirs {
        call1("library-directories", string_utf8(libdirs, -1));
    } else if let Some(libdirs) = std::env::var_os("CHEZSCHEMELIBDIRS") {
        let libdirs = libdirs.into_string().unwrap();
        call1("library-directories", string_utf8(&libdirs, -1));
    }

    if let Some(ref libexts) = args.libexts {
        call1("library-extensions", string_utf8(libexts, -1));
    } else if let Some(libexts) = std::env::var_os("CHEZSCHEMELIBEXTS") {
        let libexts = libexts.into_string().unwrap();
        call1("library-extensions", string_utf8(&libexts, -1));
    }

    if args.compile_imported_libraries {
        call1("compile-imported-libraries", r#true());
    }

    if args.disable_library_timestamps {
        call1("library-timestamp-mode", string_to_symbol("exists"));
    }

    if !args.quiet && !args.eedisable {
        let eehistory = if let Some(ref eehistory) = args.eehistory {
            eehistory.to_str().unwrap()
        } else {
            ""
        };
        enable_expeditor(eehistory)
    }

    let trailing = args.trailing.iter().map(|i| i.as_str()).collect::<Vec<_>>();

    let status = if let Some(ref script_file) = args.script {
        let script_file = script_file.to_str().unwrap();
        scheme_script(script_file, &trailing)
    } else if let Some(ref program_file) = args.program {
        let program_file = program_file.to_str().unwrap();
        scheme_program(program_file, &trailing)
    } else {
        scheme_start(&trailing)
    };

    scheme_deinit();

    std::process::exit(status);
}
