#!/usr/bin/env tarantool

local ffi = require('ffi')

ffi.cdef[[
    void lua();
]]

local br = ffi.load('brodrust')

br.lua()