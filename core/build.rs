fn main() {
    println!("HELLO");
    let target = std::env::var("TARGET").unwrap();
    if !target.contains("wasm32") && cfg!(feature = "proc-macro") {
        println!("cargo:rustc-cfg=use_proc_macro");
    }
}