#!/usr/bin/env bash

# This script builds a docker image containing all tools using nix

DOCKER_IMAGE_TAR_GZ=$(nix build .#rust-hls-container --print-out-paths --print-build-logs)

echo "New docker image is at $DOCKER_IMAGE_TAR_GZ"

echo "Loading docker image into docker"
docker load <$DOCKER_IMAGE_TAR_GZ

if test "publish" != "$1"; then
    echo "Use buildDockerImage.sh publish to publish it to dockerhub"
    exit 0
fi

echo "Publishing docker image to dockerhub not implemented yet. Probably only one command"
