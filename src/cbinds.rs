
// const root_dir: String = std::env::var("CARGO_MANIFEST_DIR").unwrap();

// const 

#[cxx::bridge]
mod ffi {

    // struct string_and_int {
    //     string: String,
    //     integer: i32,
    // }

    extern "Rust" {
        // fn return_fixed_int() -> i32;
    }

    unsafe extern "C++" {
        /* TODO fix to make relative to project dir */
        // for some reason only compiles when I use absolute path idk why 
        // include!(std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/cppsrc/include/cpp_main.hpp");
        include!("/home/aidenk/Desktop/_Kmod/cppsrc/include/cpp_main.hpp");

        // type SFML_WINDOW_WRAPPER;

        

        type ExampleWrapper;

        #[allow(dead_code)]
        fn Construct_ExampleWrapper() -> SharedPtr<ExampleWrapper>;

        #[allow(dead_code)]
        fn ExampleWrapper_get_number(ewrapper: SharedPtr<ExampleWrapper>) -> i32;
    }
}


#[cfg(test)]
mod cbind_test {
    // use super::ffi::{print_test, construct_TestClass};

    use crate::cbinds::ffi::ExampleWrapper;

    use super::ffi::Construct_ExampleWrapper;

    // #[allow(dead_code)]
    // fn test_c_print_function() {
    //     let class_ptr = construct_TestClass();
    //     assert_eq!(
    //         test_class_function(ckass_ptr),
    //         5
    //     );
    // }
    fn test_example_thtings() {
        let tmp = Construct_ExampleWrapper();
        assert!(ExampleWrapper_get_number(tmp), 115);
    }
}