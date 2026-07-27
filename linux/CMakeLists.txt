cmake_minimum_required(VERSION 3.10)

set(PROJECT_NAME "xue_hua_app_badge")
project(${PROJECT_NAME} LANGUAGES CXX)

set(PLUGIN_NAME "xue_hua_app_badge_plugin")

add_library(${PLUGIN_NAME} SHARED
  "xue_hua_app_badge_plugin.cc"
)

apply_standard_plugin_build_settings(${PLUGIN_NAME})

target_compile_definitions(${PLUGIN_NAME} PRIVATE FLUTTER_PLUGIN_IMPL)

target_include_directories(${PLUGIN_NAME} INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/include"
)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GLIB REQUIRED glib-2.0)
pkg_check_modules(GOBJECT REQUIRED gobject-2.0)
pkg_check_modules(GIO REQUIRED gio-2.0)

target_include_directories(${PLUGIN_NAME} PRIVATE
  ${GLIB_INCLUDE_DIRS}
  ${GOBJECT_INCLUDE_DIRS}
  ${GIO_INCLUDE_DIRS}
)

target_link_libraries(${PLUGIN_NAME} PRIVATE
  flutter
  ${GLIB_LIBRARIES}
  ${GOBJECT_LIBRARIES}
  ${GIO_LIBRARIES}
)

set(xue_hua_app_badge_bundled_libraries
  ""
  PARENT_SCOPE
)
