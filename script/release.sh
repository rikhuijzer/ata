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

echo ""
echo "ENSURE YOU ARE ON THE MAIN BRANCH"
echo ""

read -p 'Release notes, which will not trigger a release yet: ' NOTES

read -p "Creating a new tag, which WILL TRIGGER A RELEASE with the following release notes: \"$NOTES\". Are you sure? [y/N]" -n 1 -r
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    git tag -a $TAGNAME -m "$NOTES"
    git push origin $TAGNAME
fi
