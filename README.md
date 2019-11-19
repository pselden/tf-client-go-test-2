# tensorflow-serving-client

[![Build Status](https://travis-ci.com/figroc/tensorflow-serving-client.svg?branch=master)](https://travis-ci.com/figroc/tensorflow-serving-client)

A prebuilt tensorflow serving client from the tensorflow serving proto files.

Currently supported build:
  * C++
  * Java
  * Python
  * Node
  * Go
  * Mono
  * Rust

Check tensorflow serving project for details: https://tensorflow.github.io/serving/

### update proto files
[![Release Status](https://img.shields.io/github/v/release/tensorflow/tensorflow?label=tensorflow&sort=semver)](https://github.com/tensorflow/tensorflow)
[![Release Status](https://img.shields.io/github/v/release/tensorflow/serving?label=serving&sort=semver)](https://github.com/tensorflow/serving)
[![Release Status](https://img.shields.io/github/v/tag/figroc/tensorflow-serving-client?label=client&sort=semver)](https://github.com/figroc/tensorflow-serving-client)
```
./update.sh
```
* the desired version can be specified in the `VERSION` file

### PREREQUISITE

Grpc tools are needed for building variant packages.

Install `protobuf-compiler-grpc` on Ubuntu and `grpc` on macOS.

see `.travis.yml` for details

### build native library
[![Publish Status](https://img.shields.io/spack/v/tensorflow-serving-client)](https://github.com/spack/spack/tree/develop/var/spack/repos/builtin/packages/tensorflow-serving-client)
```
gradle cmake
```
* `libtensorflow-serving-client.a` is located in `build/dist/lib`
* **prerequisite:** `libprotobuf-dev` on Ubuntu and `protobuf` on macOS

### build java jar
[![Publish Status](https://img.shields.io/maven-central/v/io.opil/tensorflow-serving-client)](https://search.maven.org/search?q=g:io.opil%20AND%20a:tensorflow-serving-client)
```
gradle build
```
* `tensorflow-serving-client` is located in `build/libs`

### build python wheel
[![Publish Status](https://img.shields.io/pypi/v/tensorflow-serving-client-grpc)](https://pypi.org/project/tensorflow-serving-client-grpc)
```
gradle wheel
```
* `tensorflow_serving_client_grpc` is located in `build/dist`
* **prerequisite:** `setuptools`

### build node tarball
[![Publish Status](https://img.shields.io/npm/v/tensorflow-serving-client)](https://www.npmjs.com/package/tensorflow-serving-client)
```
gradle node
```
* `tensorflow-serving-client` tarball is located in `build/node`

### build go module
[![Publish Status](https://img.shields.io/github/v/tag/figroc/tensorflow-serving-client?label=go&sort=semver)](https://github.com/figroc/tensorflow-serving-client)
```
gradle golang
```
* `tensorflow-serving-client` is located in `build/generated/source/proto/main/go`
* **prerequisite:** `github.com/golang/protobuf/protoc-gen-go` for build

### build mono library
[![Publish Status](https://img.shields.io/nuget/v/tensorflow-serving-client)](https://www.nuget.org/packages/tensorflow-serving-client)
```
gradle mono
```
* `tensorflow-serving-client` is located in `build/msbuild`

### build rust crate
[![Publish Status](https://img.shields.io/crates/v/tensorflow-serving-client)](https://crates.io/crates/tensorflow-serving-client)
```
gradle rust
```
* `libtensorflow_serving_client` is located in `build/cargo`
