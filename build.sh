#!/bin/bash

set -e

gradle -DtfsV=$(cat VERSION | tr -d '\n') build
