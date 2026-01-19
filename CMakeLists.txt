cmake_minimum_required(VERSION 3.16)

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
pkg_check_modules(GLIB REQUIRED glib-2.0)

file(GLOB SRC_FILES
    "${CMAKE_CURRENT_SOURCE_DIR}/src/*.c"
)

# Executable
add_executable(aumlang ${SRC_FILES})

# Include directories
target_include_directories(aumlang PRIVATE
    ${GLIB_INCLUDE_DIRS}
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Link libraries
target_link_libraries(aumlang PRIVATE
    ${GLIB_LIBRARIES}
)

# Extra flags from pkg-config
target_compile_options(aumlang PRIVATE
    ${GLIB_CFLAGS_OTHER}
)
