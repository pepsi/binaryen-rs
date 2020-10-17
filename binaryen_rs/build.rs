extern crate bindgen;

use std::env;
use std::path::PathBuf;
// use inline_python::python;

fn main()
{
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=binaryen");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
        let bindings = std::fs::read(out_path.join("bindings.rs")).unwrap();
    // let bindings_str = std::str::from_utf8(&bindings).unwrap();
    // python!{
    //     import re
    //     lines = []
    //     lines2 = []
    //     b = 'bindings_str
    //     with open("test.txt", "w") as f: f.write(" ")
    //     lines.append(b.split("pub type BinaryenOp = i32;")[1].split("#")[0])
    //     for line in "\n".join(lines).split("\n"):
    //         if "extern \"C\"" not in line and "}" not in line:
    //             with open("test.txt", "a") as f:
    //                 source = line.strip().replace("pub fn", "").replace("-> _binaryen_op;", "")
    //                 print(source)
    //                 after = re.sub(r"(?<!^)(?=[A-Z])", "_", source).lower()
    //                 f.write(after[2:-19] + "\n")
    // }
}
