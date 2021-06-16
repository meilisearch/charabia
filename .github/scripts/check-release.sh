#!/bin/sh

# Checking if current tag matches the Cargo.toml
current_tag=$(echo $GITHUB_REF | tr -d 'refs/tags/v')
file='Cargo.toml'

file_tag="$(grep '^version = ' $file | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')"
if [ "$current_tag" != "$file_tag" ]; then
  echo "Error: the current tag does not match the version in: $file"
  echo "Found $file_tag - expected $current_tag"
  exit 1
fi

echo 'OK'
exit 0
