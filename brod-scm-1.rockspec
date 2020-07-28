package = 'brod'
version = 'scm-1'
source  = {
    url    = 'git://github.com/picodata/brod.git',
    branch = 'master',
}
description = {
    summary  = "Kafka implemented as Tarantool module",
    homepage = 'https://github.com/picodata/brod/',
    license  = 'BSD',
}
dependencies = {
    'lua >= 5.1',
}
external_dependencies = {
    TARANTOOL = {
        header = "tarantool/module.h"
    }
}
build = {
    type = 'make',
}
