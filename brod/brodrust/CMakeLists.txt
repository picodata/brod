set(FILE_RET "")
cargo_build(NAME brodrust FILE_RET)

install(FILES ${FILE_RET} TYPE "LIB")