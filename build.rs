
// extern crate cbindgen;

use std::env;

// use cmake;

// use sfml;

fn main() {

    // let root_dir = std::env::var("CARGO_MANIFEST_DIR");
	// let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

	// cbindgen::Builder::new()
	// 	.with_crate(crate_dir)
	// 	.generate()
	// 	.expect("Unable to generate bindings")
	// 	.write_to_file("rust_bindings.h")
	// ;

	cxx_build::bridge("src/cbinds.rs")
		.file("cppsrc/cpp_main.cpp")
        // .include("cppsrc/include")
		// .flag_if_supported(dst"-std=c++14")
		// .flag("-L\"\"")
		.compile("cpp_api_test")
	;

	println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/cbinds.rs");
	println!("cargo:rerun-if-changed=cppsrc/cpp_main.cpp");
	println!("cargo:rerun-if-changed=cppsrc/cpp_main.hpp");

    // #[allow(unused_variables)]
    // // let sfml_target = cmake::build("SFML");
    // let sfml_target = cmake::Config::new("SFML")
    //     .very_verbose(true)
    //     .define("RUNTIME_OUTPUT_DIRECTORY", "build/")
    //     // .out_dir("SFML/build/")
    // ;

    // // println!("cargo:rustc-link-search=native={}", sfml_target.display());


    // env::set_var("SFML_INCLUDE_DIR", "/home/aidenk/Desktop/_Kmod/SFML/include");
    // env::set_var("SFML_LIBS_DIR", "SFML/build/lib");

    // println!("{}", env::var("SFML_INCLUDE_DIR").unwrap());

}