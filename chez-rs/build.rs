fn main() {
    let (machine_code, machine_macro, machine_source) = if cfg!(all(target_os = "linux")) {
        ("ta6le", "X86_64", "i3le")
    } else {
        ("pb", "PORTABLE_BYTECODE", "pb")
    };
    let boot_dir = format!("../boot/{}", machine_code);
    let c_cources = [
        "statics",
        "segment",
        "alloc",
        "symbol",
        "intern",
        "gcwrapper",
        "gc-011",
        "gc-par",
        "gc-ocd",
        "gc-oce",
        "number",
        "schsig",
        "io",
        "new-io",
        "compress-io",
        "fasl",
        "vfasl",
        "print",
        "stats",
        "foreign",
        "prim",
        "prim5",
        "flushcache",
        "schlib",
        "thread",
        "expeditor",
        "scheme",
        "random",
        "ffi",
        "self-exe",
        machine_source,
    ]
    .map(|i| format!("../c/{}.c", i));
    c_cources
        .iter()
        .for_each(|i| println!("cargo::rerun-if-changed={}", i));
    [&boot_dir, "config.h"]
        .iter()
        .for_each(|i| println!("cargo::rerun-if-changed={}", i));
    ["curses", "lz4", "z"]
        .iter()
        .for_each(|i| println!("cargo::rustc-link-lib={}", i));
    cc::Build::new()
        .files(c_cources)
        .includes([
            &std::env::var("DEP_Z_INCLUDE").unwrap(),
            &std::env::var("DEP_LZ4_INCLUDE").unwrap(),
            &boot_dir,
            ".",
        ])
        .flag("-pthread")
        .flag("-m64")
        .flag("-msse2")
        .flag("-Wextra")
        .flag("-Wall")
        .flag("-Wno-unused-parameter")
        .pic(true)
        .define(machine_macro, None)
        .compile("kernel");
}
