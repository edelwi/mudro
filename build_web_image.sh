#!/bin/sh
# if image was builded locally
TAG=1.0.0
docker build -t edelwi/mudro:${TAG} .
docker push edelwi/mudro:${TAG}
