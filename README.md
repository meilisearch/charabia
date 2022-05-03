# Charabia
Library used by Meilisearch to tokenize queries and documents

## Role

The tokenizer’s role is to take a sentence or phrase and split it into smaller units of language, called tokens. It finds and retrieves all the words in a string based on the language’s particularities.

## Details

Charabia is modular. It goes field by field, determining the most likely language for the field and running a different pipeline for each language.

## Supported languages

**Charabia is multilingual**, featuring optimized support for:


|  Script - Language  |                           specialized segmentation                            | specialized normalization | Segmentation Performance level | Tokenization Performance level |
|---------------------|-------------------------------------------------------------------------------|---------------------------|-------------------|---|
| **Latin** - **Any** | ✅ [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) | ✅ lowercase + deunicode            | 🟩 ~45MiB/sec    | 🟨 ~24MiB/sec    |
| **Chinese** - **CMN** 🇨🇳 | ✅ [jieba](https://github.com/messense/jieba-rs) | ✅ traditional-to-simplified conversion | 🟨 ~21MiB/sec    | 🟧 ~9MiB/sec    |

We aim to provide global language support, and your feedback helps us [move closer to that goal](https://docs.meilisearch.com/learn/advanced/language.html#improving-our-language-support). If you notice inconsistencies in your search results or the way your documents are processed, please open an issue on our [GitHub repository](https://github.com/meilisearch/tokenizer/issues/new/choose).

### About Performance level

Performances are based on the throughput (MiB/sec) of the tokenizer (computed on a MacBook Pro 2021 - Apple M1 Pro) using jemalloc:
- 0️⃣⬛️: 0   -> 1   MiB/sec
- 1️⃣🟥: 1   -> 5   MiB/sec
- 2️⃣🟧: 5   -> 12  MiB/sec
- 3️⃣🟨: 12  -> 35  MiB/sec
- 4️⃣🟩: 35  -> 75  MiB/sec
- 5️⃣🟪: 75  -> ... MiB/sec