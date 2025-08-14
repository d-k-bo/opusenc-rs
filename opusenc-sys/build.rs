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

    let mut clang_args = vec!();

    #[cfg(target_os = "macos")] {
        add_include("/opt/homebrew/include", &mut clang_args);
        add_include("/usr/local/include", &mut clang_args);
        add_include("/usr/include", &mut clang_args);

        add_link_search(PathBuf::from("/opt/homebrew/lib"));
        add_link_search(PathBuf::from("/usr/local/lib"));
    }

    #[cfg(not(target_os = "macos"))] {
        add_include("/usr/include", &mut clang_args);
        add_include("/usr/local/include", &mut clang_args);

        add_link_search(PathBuf::from("/usr/lib"));
        add_link_search(PathBuf::from("/usr/local/lib"));
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
