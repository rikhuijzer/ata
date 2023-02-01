#!/usr/bin/env bash

set -e

# Thanks to https://stackoverflow.com/questions/1841341.
git tag -l | xargs git tag -d
git fetch --tags
