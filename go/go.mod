module github.com/figroc/tensorflow-serving-client/go

go 1.12

require (
	github.com/golang/protobuf v1.3.2
	google.golang.org/grpc v1.26.0
	tensorflow v0.0.0-00010101000000-000000000000 // indirect
	tensorflow_serving v0.0.0-00010101000000-000000000000
)

replace tensorflow_serving => ./tensorflow_serving

replace tensorflow => ./tensorflow
