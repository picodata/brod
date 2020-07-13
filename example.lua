#!/usr/bin/env tarantool

local ffi = require('ffi')

ffi.cdef[[
    void brod_listen(const char *addr);
    void test_ffi(const char *addr);
]]

local br = ffi.load('brodrust')

local addr = "127.0.0.1:8080"
br.brod_listen(addr)