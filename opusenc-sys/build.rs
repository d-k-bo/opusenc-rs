use std::path::PathBuf;

fn add_include<P: AsRef<std::path::Path>>(path: P, args: &mut Vec<String>) {
  if path.as_ref().exists() {
    args.push("-isystem".into());
    args.push(path.as_ref().display().to_string());
  }
}

fn add_link_search<P: AsRef<std::path::Path>>(path: P) {
  if path.as_ref().exists() {
    println!("cargo:rustc-link-search={}", path.as_ref().display())
  }
}

fn main() {
    println!("cargo:rustc-link-lib=opusenc");
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut clang_args = Vec::new();
    if let Ok(opus_include) = std::env::var("OPUS_INCLUDE_DIR") {
        clang_args.push("-isystem".into());
        clang_args.push(opus_include);
    }

    #[cfg(target_os = "macos")] {
        let include_paths= [
            "/opt/homebrew/include",
            "/usr/local/include", 
            "/usr/include",
            "/opt/homebrew/include/opus", 
            "/usr/local/include/opus",
            "/usr/include/opus",
        ];

        let lib_paths = [
            "/opt/homebrew/lib",
            "/usr/local/lib"
        ];

        include_paths.iter().for_each(|path| add_include(path, &mut clang_args));
        lib_paths.iter().for_each(add_link_search);
    }

    #[cfg(not(target_os = "macos"))] {
        let include_paths= [
            "/usr/include",
            "/usr/local/include", 
            "/usr/include/opus", 
            "/usr/local/include/opus",
        ];

        let lib_paths = [
            "/usr/lib",
            "/usr/local/lib"
        ];

        include_paths.iter().for_each(|path| add_include(path, &mut clang_args));
        lib_paths.iter().for_each(add_link_search);
    }

    if let Ok(opus_lib) = std::env::var("OPUS_LIB_DIR") {
      println!("cargo:rustc-link-search={opus_lib}")
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&clang_args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
