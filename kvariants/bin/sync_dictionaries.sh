#!/bin/sh

dir="$(dirname "$0")"
dictionaries="$dir/../dictionaries"

. $dir/clean.sh

curl https://raw.githubusercontent.com/hfhchan/irg/master/kVariants.txt > $dictionaries/source/kVariants.tsv

