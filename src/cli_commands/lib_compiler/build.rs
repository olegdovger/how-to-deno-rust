use std::process::Command;

pub fn build(lib_path: &str) {
    println!("\n Build library \"{}\"\n", &lib_path);

    if cfg!(target_os = "windows") {
        let _prog = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .args(&["wasm-pack build", &lib_path])
                .output()
                .unwrap()
        } else {
            //todo: test it from docker
            Command::new("wasm-pack build")
                .arg(&lib_path)
                .output()
                .unwrap()
        };
    };
}
