set(csrcs
    "src/hello_world.c"
)

set(inc_dirs
    "lib"
)
set(priv_inc_dirs
    "src"
)

idf_component_register( SRCS ${csrcs}
                        INCLUDE_DIRS ${inc_dirs} "."
                        PRIV_INCLUDE_DIRS ${priv_inc_dirs}
                        REQUIRES # Here we add components like in a regular esp32 project 
                       )

target_compile_options(${COMPONENT_LIB} PUBLIC "-DCONSTANTS_STORAGE_NAME=\"/storage/constants.json\"")
target_compile_options(${COMPONENT_LIB} PUBLIC "--param=max-vartrack-size=0") 
      
if (CMAKE_BUILD_TYPE MATCHES DEBUG)
    target_compile_options(${COMPONENT_LIB} PRIVATE -ggdb -Og -DDEBUG -DC99)
elseif (CMAKE_BUILD_TYPE MATCHES TEST)
    target_compile_options(${COMPONENT_LIB} PRIVATE -ggdb -Og -DTEST -DC99)
    target_link_libraries(${COMPONENT_LIB} PUBLIC -DTEST)
elseif (CMAKE_BUILD_TYPE MATCHES RELEASE)
    target_compile_options(${COMPONENT_LIB} PRIVATE -O2 -DC99)
else()
    target_compile_options(${COMPONENT_LIB} PRIVATE -O2 -DC99)
endif()

set(my_cflags_public -Wall -Warray-bounds)
message(STATUS "my_cflags_public: ${my_cflags_public}")