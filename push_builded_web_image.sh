#!/bin/sh
# if image was builded locally
TAG=1.0.0
docker tag mudro_app edelwi/mudro:${TAG}
# push to docker hub
docker push edelwi/mudro:${TAG}
