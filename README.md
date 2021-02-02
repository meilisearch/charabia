# Tokenizer
Library used by Meilisearch to tokenize queries and documents

## Role

The tokenizer’s role is to take a sentence or phrase and split it into smaller units of language, called tokens. It finds and retrieves all the words in a string based on the language’s particularities.  

## Details

MeiliSearch’s tokenizer is modular. It goes field by field, determining the most likely language for the field and running a different pipeline for each language.

Pipelines include language-specific processes. For example, the Chinese pipeline converts all text into simplified Chinese before tokenization, allowing a single search query to give results in both traditional and simplified Chinese.

If you'd like to read more about the tokenizer design, check out the [feature specification](https://github.com/meilisearch/specifications/blob/master/text/0001-script-based-tokenizer.md).

## Supported languages

**MeiliSearch is multilingual**, featuring optimized support for:

- **Any language that uses whitespace to separate words**
- **Chinese** 🇨🇳 (through [Jieba](https://github.com/messense/jieba-rs))

We aim to provide global language support, and your feedback helps us [move closer to that goal](https://docs.meilisearch.com/learn/advanced/language.html#improving-our-language-support). If you notice inconsistencies in your search results or the way your documents are processed, please open an issue on our [GitHub repository](https://github.com/meilisearch/MeiliSearch/issues/new/choose).
