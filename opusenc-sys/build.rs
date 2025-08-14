use std::path::PathBuf;

fn add_include<P: AsRef<std::path::Path>>(path: P, args: &mut Vec<String>) {
  if path.as_ref().exists() {
    args.push("-isystem".into());
    args.push(path.as_ref().display());
  }
}

fn add_link_search<P: AsRef<std::path::Path>>(path: P) {
  if path.as_ref().exists() {
    println!("cargo:rustc-link-search={}", path.as_ref().display())
  }
}


fn main() {
    #[cfg(target_os = "macos")] {
        if PathBuf::from("/opt/homebrew/lib").exists() {
            println!("cargo:rustc-link-search=/opt/homebrew/lib");
        }
        if PathBuf::from("/usr/local/lib").exists() {
            println!("cargo:rustc-link-search=/usr/local/lib");
        }
    }

    #[cfg(not(target_os = "macos"))] {
        println!("cargo:rustc-link-search=/usr/local/lib");
    }


    println!("cargo:rustc-link-lib=opusenc");
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut clang_args = vec!();


    #[cfg(target_os = "macos")] {
        add_include("/opt/homebrew/include", &mut clang_args);
        add_include("/usr/local/include", &mut clang_args);
        add_include("/usr/include", &mut clang_args);
    }

    #[cfg(not(target_os = "macos"))] {
        add_include("/usr/include", &mut clang_args);
        add_include("/usr/local/include", &mut clang_args);
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
