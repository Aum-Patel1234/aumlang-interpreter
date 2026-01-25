cmake_minimum_required(VERSION 3.16)
add_compile_definitions(_POSIX_C_SOURCE=200809L)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(aumlang
    VERSION 0.1.0
    DESCRIPTION "AumLang - A toy Interpreted programming language written in C"
    LANGUAGES C
)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

if (CMAKE_C_COMPILER_ID MATCHES "GNU|Clang")
    add_compile_options(
        -Wall
        -Wextra
        -Wpedantic
        -Wshadow
        -Wconversion
        -Wno-unused-parameter
    )
endif()

# Find GLib
find_package(PkgConfig REQUIRED)
pkg_check_modules(GLIB REQUIRED IMPORTED_TARGET glib-2.0)


file(GLOB TEST_FILES
    "${CMAKE_CURRENT_SOURCE_DIR}/test/*.c"
)
file(GLOB SRC_FILES
    "${CMAKE_CURRENT_SOURCE_DIR}/src/*.c"
)

set(SRC_FILES_NO_MAIN ${SRC_FILES})
list(FILTER SRC_FILES_NO_MAIN EXCLUDE REGEX ".*/main.c$")

# Executable
add_executable(aumlang ${SRC_FILES})
add_executable(aumlang_test
    ${SRC_FILES_NO_MAIN}
    ${TEST_FILES}
)

target_link_libraries(aumlang PRIVATE PkgConfig::GLIB)
target_link_libraries(aumlang_test PRIVATE PkgConfig::GLIB)
