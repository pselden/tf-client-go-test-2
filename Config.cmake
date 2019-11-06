find_package(PkgConfig REQUIRED)
pkg_search_module(GRPC REQUIRED grpc)
pkg_search_module(GRPCPP REQUIRED grpc++>=@TFSCLIENT_VERSION_GRPC@)

include(CMakeFindDependencyMacro)
find_dependency(Protobuf @TFSCLIENT_VERSION_PROTO@ REQUIRED)

include("${CMAKE_CURRENT_LIST_DIR}/@TFSCLIENT_TARGETF@")
