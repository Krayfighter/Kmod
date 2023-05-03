
#pragma once

#include <memory>


class ExampleWrapper {
    public:
        ExampleWrapper();
        int get_ewrapped_value();
    private:
        int ewrapped_value;
};


std::shared_ptr<ExampleWrapper> Construct_ExampleWrapper();

int ExampleWrapper_get_number(std::shared_ptr<ExampleWrapper> ewrapper);

