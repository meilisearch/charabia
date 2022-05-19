# Charabia
Library used by Meilisearch to tokenize queries and documents
We choose the name of this repository because it starts with char which was done in the same mood as discord or meili, giving the name of the problem we want to solve.
Personally, I don’t feel like it’s an offensive word, but more a funny pun with “char”.
Moreover, other tokenizers don’t always have an understandable name, for instance [lindera](https://github.com/lindera-morphology/lindera) maintained by [@minoru_osuka](https://twitter.com/minoru_osuka) or even [jieba](https://github.com/messense/jieba-rs).
I hope my explanation was clear enough and I hope the name will not discourage you to use or even contribute to the project! 😊

## Role

The tokenizer’s role is to take a sentence or phrase and split it into smaller units of language, called tokens. It finds and retrieves all the words in a string based on the language’s particularities.

## Details

Charabia provides a simple API to segment, normalize, or tokenize (segment + normalize) a text of a specific language by detecting its Script/Language and choosing the specialized pipeline for it.

## Supported languages

**Charabia is multilingual**, featuring optimized support for:


|  Script - Language  |                           specialized segmentation                            | specialized normalization | Segmentation Performance level | Tokenization Performance level |
|---------------------|-------------------------------------------------------------------------------|---------------------------|-------------------|---|
| **Latin** - **Any** | ✅ [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) | ✅ lowercase + deunicode            | 🟨 ~28MiB/sec    | 🟨 ~12MiB/sec    |
| **Chinese** - **CMN** 🇨🇳 | ✅ [jieba](https://github.com/messense/jieba-rs) | ✅ traditional-to-simplified conversion | 🟨 ~17MiB/sec    | 🟧 ~9MiB/sec    |
| **Hebrew** 🇮🇱 | ✅ [unicode-segmentation](https://github.com/messense/jieba-rs) | ✅ diacritics removal  | 🟩 ~45MiB/sec    | 🟨 ~17MiB/sec    |

We aim to provide global language support, and your feedback helps us [move closer to that goal](https://docs.meilisearch.com/learn/advanced/language.html#improving-our-language-support). If you notice inconsistencies in your search results or the way your documents are processed, please open an issue on our [GitHub repository](https://github.com/meilisearch/charabia/issues/new/choose).

If you have a particular need that charabia does not support, please share it in the product repository by creating a [dedicated discussion](https://github.com/meilisearch/product/discussions?discussions_q=label%3Aproduct%3Acore%3Atokenizer).

### About Performance level

Performances are based on the throughput (MiB/sec) of the tokenizer (computed on a MacBook Pro 2021 - Apple M1 Pro) using jemalloc:
- 0️⃣⬛️: 0   -> 1   MiB/sec
- 1️⃣🟥: 1   -> 5   MiB/sec
- 2️⃣🟧: 5   -> 12  MiB/sec
- 3️⃣🟨: 12  -> 35  MiB/sec
- 4️⃣🟩: 35  -> 75  MiB/sec
- 5️⃣🟪: 75 MiB/sec or more

## Examples

#### Tokenization

```rust
use charabia::Tokenize;

let orig = "Thé quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";

// tokenize the text.
let mut tokens = orig.tokenize();

let token = tokens.next().unwrap();
// the lemma into the token is normalized: `Thé` became `the`.
assert_eq!(token.lemma(), "the");
// token is classfied as a word
assert!(token.is_word());

let token = tokens.next().unwrap();
assert_eq!(token.lemma(), " ");
// token is classfied as a separator
assert!(token.is_separator());
```

#### Segmentation

```rust
use charabia::Segment;

let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";

// segment the text.
let mut segments = orig.segment_str();

assert_eq!(segments.next(), Some("The"));
assert_eq!(segments.next(), Some(" "));
assert_eq!(segments.next(), Some("quick"));
```
