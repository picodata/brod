/* mycmodule - a simple Tarantool module */
#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>

#include <module.h>

static int
myfun(lua_State *L)
{
    if (lua_gettop(L) < 1)
        return luaL_error(L, "Usage: myfun(name)");

    /* Get first argument */
    const char *name = lua_tostring(L, 1);

    /* Push one result to Lua stack */
    lua_pushfstring(L, "Hello, %s", name);
    return 1; /* the function returns one result */
}

LUA_API int
luaopen_brod(lua_State *L)
{
    static const struct luaL_Reg reg[] = {
        { "myfun", myfun },
        { NULL, NULL }
    };
    luaL_register(L, "brod", reg);
    return 1;
}
