# tensorflow-serving-client-java

A prebuilt tensorflow serving client from the tensorflow serving proto files

Check tensorflow serving project for details: https://tensorflow.github.io/serving/

## update proto files

```
./proto.sh
```
* the desired version can be specified in the `VERSION` file

## build jar file

```
gradle build
```
* `tensorflow-serving-client` is located in `build/libs`
