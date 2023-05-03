

// // TODO
// // this should be configured at compile time
// #define PROJECT_SRC_DIR "home/aidenk/Desktop/_Kmod/"

// #include "stdio.h"
#include "include/cpp_main.hpp"
// #include PROJECT_SRC_DIR+"/target/cbinds.rs.h"
#include <memory>


ExampleWrapper::ExampleWrapper() {
    ewrapped_value = 115;
}
int ExampleWrapper::get_ewrapped_value() {
    return ewrapped_value;
}



std::shared_ptr<ExampleWrapper> Construct_ExampleWrapper() {
    return std::shared_ptr<ExampleWrapper>(new ExampleWrapper());
}

int ExampleWrapper_get_number(std::shared_ptr<ExampleWrapper> ewrapper) {
    return ewrapper->get_ewrapped_value();
}
