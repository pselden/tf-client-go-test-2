include(CMakeFindDependencyMacro)
find_dependency(Protobuf @TFSCLIENT_VERSION_PROTO@ REQUIRED)

find_dependency(gRPC @TFSCLIENT_VERSOIN_GRPC@ CONFIG)
if(NOT gRPC_FOUND)
  find_package(PkgConfig REQUIRED)
  pkg_search_module(GRPC REQUIRED grpc)
  pkg_search_module(GRPCPP REQUIRED grpc++>=@TFSCLIENT_VERSION_GRPC@)
endif()

include("${CMAKE_CURRENT_LIST_DIR}/@TFSCLIENT_TARGETF@")
