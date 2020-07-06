# Build
if (APPLE)
    set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -undefined suppress -flat_namespace")
endif(APPLE)

enable_language(Rust)
include(CMakeCargo)
add_subdirectory(brodrust)

# Install
# install(TARGETS brod LIBRARY DESTINATION ${TARANTOOL_INSTALL_LIBDIR})
