#!/bin/bash

gradle -Dtfs=$(cat VERSION | tr -d '\n') build
