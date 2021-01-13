# Tokenizer
Library used by Meilisearch to tokenize queries and documents

## Role

The tokenizer’s role is to take a sentence or phrase and split it into smaller units of language, called tokens. It finds and retrieves all the words in a string based on the language’s particularities.  

## Details

MeiliSearch’s tokenizer is modular. It goes field by field, determining the most likely language for the field and running a different pipeline for each language.

The tokenizer identifies words made of one or more Hanzi / Kanji characters. In addition, a single search query will give results in both traditional and simplified Chinese.

## Supported languages

Currently, it supports any language that uses the Latin alphabet, as well as Chinese, Japanese, and Korean.
