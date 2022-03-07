cmake_minimum_required(VERSION 3.0.0)
project(backend VERSION 0.1.0)

SET(CMAKE_TOOLCHAIN_FILE E:/Workspaces/Code/vcpkg/scripts/buildsystems/vcpkg.cmake)

find_package(fmt CONFIG REQUIRED)
find_package(drogon CONFIG REQUIRED)
find_package(openssl CONFIG REQUIRED)

include_directories(.)
add_subdirectory(.)

add_executable(backend main.cpp)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})

include(CPack)

target_link_libraries(backend PRIVATE fmt::fmt)