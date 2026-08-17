cmake_minimum_required(VERSION 3.22.1)
project(braxon_android LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library(braxon_android SHARED main.cpp ${ANDROID_NDK}/sources/android/native_app_glue/android_native_app_glue.c)
target_include_directories(braxon_android PRIVATE ${ANDROID_NDK}/sources/android/native_app_glue)
target_compile_options(braxon_android PRIVATE -Wall -Wextra -Werror -fno-exceptions -fno-rtti)
target_link_libraries(braxon_android android log)
