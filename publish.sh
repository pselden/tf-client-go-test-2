#!/bin/bash -e

rm -rf build \
  && gradle build \
  && gradle -b ossrh.gradle publish
rm -rf build \
  && gradle wheel \
  && twine upload -s -i 50E54CCB build/dist/tensorflow_serving_client_*
sed -i.origin "1,/\"version\": \"0.0./ s/0.0/$(cat VERSION)/" package.json \
  && npm publish \
  && mv -f package.json.origin package.json
