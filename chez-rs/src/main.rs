fn main() {
    let args = std::env::args()
        .map(|i| std::ffi::CString::new(i).expect("CString::new failed"))
        .collect::<Vec<_>>();
    kernel::main(&args);
}
