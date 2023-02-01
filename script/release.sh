#!/usr/bin/env bash

#
# Trigger a release
#

set -e

# We have to run this locally because tags created from workflows do not
# trigger new workflows.
# "This prevents you from accidentally creating recursive workflow runs."

METADATA="$(cargo metadata --format-version=1 --no-deps)"
VERSION="$(echo $METADATA | jq -r '.packages[0].version')"
echo "VERSION: $VERSION"
TAGNAME="v$VERSION"
echo "TAGNAME: $TAGNAME"
git tag -a $TAGNAME -m "Tag for $TAGNAME"
git push origin $TAGNAME
