cmake_minimum_required(VERSION 3.23)

include(FetchContent)

FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.5 # Optionally specify a commit hash, version tag or branch here
)
# Set any global configuration variables such as `Rust_TOOLCHAIN` before this line!
FetchContent_MakeAvailable(Corrosion)

project(mk_pass CXX)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(
    MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
    CRATES mk-pass-cpp
)

# Add a generated header file which will be exported
target_include_directories(mk_pass
    INTERFACE
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)
target_sources(mk_pass
    INTERFACE
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mk_pass.hpp>
    $<INSTALL_INTERFACE:include/mk_pass.hpp>
)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
option(MK_PASS_TESTS "Include unit tests in the build configuration." OFF)

if (${MK_PASS_TESTS})
    message(STATUS "Including unit tests")
    add_subdirectory(tests)
endif()

add_executable(mk-pass src/main.cpp)
target_link_libraries(mk-pass mk_pass)

install(TARGETS mk_pass
    DESTINATION lib
)

install(FILES
        include/mk_pass.hpp
    DESTINATION include/mk_pass
)
