# Build
if (APPLE)
    set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -undefined suppress -flat_namespace")
endif(APPLE)

add_library(brod SHARED brod.c)
set_target_properties(brod
        PROPERTIES
            PREFIX ""
            SUFFIX ".so"
            OUTPUT_NAME "brod")

# Install
install(TARGETS brod LIBRARY DESTINATION ${TARANTOOL_INSTALL_LIBDIR})
# install(FILES server/init.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http/server)
# install(FILES server/tsgi_adapter.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http/server)
# install(FILES nginx_server/init.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http/nginx_server)
# install(FILES router/init.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http/router)
# install(FILES router/fs.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http/router)
# install(FILES router/matching.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http/router)
# install(FILES router/middleware.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http/router)
# install(FILES router/request.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http/router)
# install(FILES router/response.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http/router)
# install(FILES tsgi.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http)
# install(FILES utils.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http)
# install(FILES mime_types.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http)
# install(FILES codes.lua DESTINATION ${TARANTOOL_INSTALL_LUADIR}/http)
