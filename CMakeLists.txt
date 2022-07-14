cmake_minimum_required(VERSION 3.10)

# set the project name
project(Tutorial VERSION 0.0)


set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED true)

# add the executable
add_executable(vm_test src/main.cc src/vm.cc)
