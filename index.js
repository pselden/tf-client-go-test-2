const fs = require("fs");
const path = require("path");
const grpc = require("grpc");
const proto = require("@grpc/proto-loader");

const def = { keepCase: true, longs: String, enums: String, defaults: true, oneofs: true,
              includeDirs: [path.join(__dirname, "proto")] };
const dir = path.join(__dirname, "proto/tensorflow_serving/apis");
const api = fs.readdirSync(dir)
              .filter(f => f.endsWith("_service.proto"))
              .map(f => path.join(dir, f));
module.exports = grpc.loadPackageDefinition(proto.loadSync(api, def));
