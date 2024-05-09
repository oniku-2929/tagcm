#!/bin/bash

version=${CURRENT_VERSION}
if [ -z "$version" ]; then
  echo "Error: CURRENT_VERSION is not set."
  exit 1
fi

IFS='.' read -r major minor patch <<< "$version"

for arg in "$@"
do
  case $arg in
    major)
      ((major++))
      minor=0
      patch=0
      ;;
    minor)
      ((minor++))
      patch=0
      ;;
    patch)
      ((patch++))
      ;;
    *)
      echo "Usage: $0 [major] [minor] [patch]"
      exit 1
      ;;
  esac
done

new_version="${major}.${minor}.${patch}"
echo $new_version