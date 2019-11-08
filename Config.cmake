include(CMakeFindDependencyMacro)
find_dependency(Protobuf 3.10.0 REQUIRED)

find_dependency(gRPC 1.23.0 CONFIG)
if(NOT gRPC_FOUND)
  find_package(PkgConfig REQUIRED)
  pkg_search_module(GRPC REQUIRED grpc)
  pkg_search_module(GRPCPP REQUIRED grpc++>=1.23.0)
endif()

include("${CMAKE_CURRENT_LIST_DIR}/tensorflow-serving-client-targets.cmake")
