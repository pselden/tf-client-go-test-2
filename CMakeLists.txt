cmake_minimum_required(VERSION 3.5)

project(tensorflow-serving-client)

find_package(Protobuf REQUIRED)

if(MSVC)
  add_definitions(-D_WIN32_WINNT=0x600)
else()
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")
endif()

set(PROTO_GENERATED "${CMAKE_SOURCE_DIR}/build/generated/source/proto/main/cpp")
file(STRINGS VERSION rVERSION)
file(GLOB_RECURSE SOURCES "${PROTO_GENERATED}/*.cc")

include_directories(${PROTO_GENERATED})
include_directories(${PROTOBUF_INCLUDE_DIRS})
add_library(tensorflow_serving_client STATIC ${SOURCES})
set_target_properties(tensorflow_serving_client PROPERTIES VERSION ${rVERSION})
target_link_libraries(tensorflow_serving_client ${PROTOBUF_LIBRARIES})

install(TARGETS tensorflow_serving_client DESTINATION lib/static)
install(DIRECTORY "${PROTO_GENERATED}/" DESTINATION include FILES_MATCHING PATTERN "*.h")
