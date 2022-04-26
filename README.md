# Tokenizer
Library used by Meilisearch to tokenize queries and documents

## Role

The tokenizer’s role is to take a sentence or phrase and split it into smaller units of language, called tokens. It finds and retrieves all the words in a string based on the language’s particularities.

## Details

Meilisearch’s tokenizer is modular. It goes field by field, determining the most likely language for the field and running a different pipeline for each language.

## Supported languages

**Meilisearch is multilingual**, featuring optimized support for:


|  Script - Language  |                           specialized segmentation                            | specialized normalization | Performance level |   |
|---------------------|-------------------------------------------------------------------------------|---------------------------|-------------------|---|
| **Latin** - **Any** | ✅ [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) | ✅ lowercase              | 🟩 ~25MiB/sec    |   |


We aim to provide global language support, and your feedback helps us [move closer to that goal](https://docs.meilisearch.com/learn/advanced/language.html#improving-our-language-support). If you notice inconsistencies in your search results or the way your documents are processed, please open an issue on our [GitHub repository](https://github.com/meilisearch/tokenizer/issues/new/choose).
