# Tokenizer
Library used by Meilisearch to tokenize queries and documents

## Role

The tokenizer’s role is to take a sentence or phrase and split it into smaller units of language, called tokens. It finds and retrieves all the words in a string based on the language’s particularities.  

## Details

MeiliSearch’s tokenizer is modular. It goes field by field, determining the most likely language for the field and running a different pipeline for each language.

Pipelines include language-specific processes. For example, the Chinese pipeline converts all text into simplified Chinese before tokenization, allowing a single search query to give results in both traditional and simplified Chinese.

If you'd like to read more about the tokenizer design, check out the [feature specification](https://github.com/meilisearch/specifications/blob/master/text/0001-script-based-tokenizer.md).

## Supported languages

Currently, the MeiliSearch tokenizer is optimized for four languages:

- **English**  🇬🇧
- **Chinese** 🇨🇳
- **Japanese** 🇯🇵
- **Korean** 🇰🇷

It also supports any language that uses the Latin alphabet. **Results may vary in languages with long compound words, such as German.**
