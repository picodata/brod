#!/usr/bin/env tarantool

require('brod')

box_info()
listen("127.0.0.1:7878")