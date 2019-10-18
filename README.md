# tensorflow-serving-client

[![Build Status](https://travis-ci.com/figroc/tensorflow-serving-client.svg?branch=master)](https://travis-ci.com/figroc/tensorflow-serving-client)

A prebuilt tensorflow serving client from the tensorflow serving proto files

Check tensorflow serving project for details: https://tensorflow.github.io/serving/

### update proto files
```
./update.sh
```
* the desired version can be specified in the `VERSION` file

### build jar file
```
gradle build
```
* `tensorflow-serving-client` is located in `build/libs`

### build wheel file
```
gradle wheel
```
* `tensorflow_serving_client` is located in `dist`
