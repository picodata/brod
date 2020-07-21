#!/usr/bin/env tarantool

local ffi = require('ffi')

ffi.cdef[[
    void brod_listen(char * addr);
]]

local br = ffi.load('brodrust')

local addr = "127.0.0.1:7878"
local c_addr = ffi.new("char[?]", #addr)
ffi.copy(c_addr, addr)

br.brod_listen(c_addr)