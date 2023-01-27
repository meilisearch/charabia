#!/bin/sh

dir="$(dirname "$0")"
dictionaries="$dir/../dictionaries"

rm -f $dictionaries/source/*.tsv
rm -f $dictionaries/compressed/*.csv
