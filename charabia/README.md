# Charabia
Library used by Meilisearch to tokenize queries and documents

## Role

The tokenizer’s role is to take a sentence or phrase and split it into smaller units of language, called tokens. It finds and retrieves all the words in a string based on the language’s particularities.

## Details

Charabia provides a simple API to segment, normalize, or tokenize (segment + normalize) a text of a specific language by detecting its Script/Language and choosing the specialized pipeline for it.

## Supported languages

**Charabia is multilingual**, featuring optimized support for:


|  Script / Language  |                           specialized segmentation                            | specialized normalization | Segmentation Performance level | Tokenization Performance level |
|---------------------|-------------------------------------------------------------------------------|---------------------------|-------------------|---|
| **Latin** | ✅ CamelCase segmentation | ✅ [compatibility decomposition](https://unicode.org/reports/tr15/) + lowercase + [nonspacing-marks](https://www.compart.com/en/unicode/category/Mn) removal + `Ð vs Đ` spoofing normalization + `ı` normalization | 🟩 ~23MiB/sec    | 🟨 ~9MiB/sec    |
| **Latin** - **German** | ✅ CamelCase segmentation + German word segmentation | ✅ [compatibility decomposition](https://unicode.org/reports/tr15/) + lowercase + [nonspacing-marks](https://www.compart.com/en/unicode/category/Mn) removal + `Ð vs Đ` spoofing normalization + `ı` normalization | 🟩 ~23MiB/sec    | 🟨 ~9MiB/sec    |
| **Greek** | ❌ | ✅ [compatibility decomposition](https://unicode.org/reports/tr15/) + lowercase + final sigma normalization         | 🟩 ~27MiB/sec    | 🟨 ~8MiB/sec    |
| **Cyrillic** - **Georgian** | ❌ | ✅ [compatibility decomposition](https://unicode.org/reports/tr15/) + lowercase          | 🟩 ~27MiB/sec    | 🟨 ~9MiB/sec    |
| **Chinese** **CMN** 🇨🇳 | ✅ [jieba](https://github.com/messense/jieba-rs) | ✅ [compatibility decomposition](https://unicode.org/reports/tr15/) + kvariant conversion | 🟨 ~10MiB/sec    | 🟧 ~5MiB/sec    |
| **Hebrew** 🇮🇱 | ❌ | ✅ [compatibility decomposition](https://unicode.org/reports/tr15/) + [nonspacing-marks](https://www.compart.com/en/unicode/category/Mn) removal  | 🟩 ~33MiB/sec    | 🟨 ~11MiB/sec    |
| **Arabic**  | ✅ `ال` segmentation | ✅ [compatibility decomposition](https://unicode.org/reports/tr15/) + [nonspacing-marks](https://www.compart.com/en/unicode/category/Mn) removal + [Tatweel, Alef, Yeh, and Taa Marbuta normalization]  + Persian digits to ASCII conversion (۰-۹ → 0-9) | 🟩 ~36MiB/sec    | 🟨 ~11MiB/sec    |
| **Japanese** 🇯🇵 | ✅ [lindera](https://github.com/lindera-morphology/lindera) IPA-dict | ❌ [compatibility decomposition](https://unicode.org/reports/tr15/) | 🟧 ~3MiB/sec    | 🟧 ~3MiB/sec    |
| **Korean** 🇰🇷 | ✅ [lindera](https://github.com/lindera-morphology/lindera) KO-dict | ❌ [compatibility decomposition](https://unicode.org/reports/tr15/) | 🟥 ~2MiB/sec    | 🟥 ~2MiB/sec    |
| **Thai** 🇹🇭 | ✅ [dictionary based](https://github.com/PyThaiNLP/nlpo3) | ✅ [compatibility decomposition](https://unicode.org/reports/tr15/) + Sara Am (`ำ`) recomposition, vowels and tone marks are kept | 🟩 ~22MiB/sec    | 🟨 ~11MiB/sec    |
| **Khmer** 🇰🇭 | ✅ dictionary based | ✅ [compatibility decomposition](https://unicode.org/reports/tr15/) | 🟧 ~7MiB/sec    | 🟧 ~5MiB/sec    |

We aim to provide global language support, and your feedback helps us [move closer to that goal](https://docs.meilisearch.com/learn/advanced/language.html#improving-our-language-support). If you notice inconsistencies in your search results or the way your documents are processed, please open an issue on our [GitHub repository](https://github.com/meilisearch/charabia/issues/new/choose).

If you have a particular need that charabia does not support, please share it in the product repository by creating a [dedicated discussion](https://github.com/meilisearch/product/discussions?discussions_q=label%3Aproduct%3Acore%3Atokenizer).

### About Performance level

Performances are based on the throughput (MiB/sec) of the tokenizer (computed on a [scaleway Elastic Metal server EM-A410X-SSD](https://www.scaleway.com/en/pricing/) - CPU: Intel Xeon E5 1650 - RAM: 64 Go) using jemalloc:
- 0️⃣⬛️:  0  ->  1  MiB/sec
- 1️⃣🟥:  1  ->  3  MiB/sec
- 2️⃣🟧:  3  ->  8  MiB/sec
- 3️⃣🟨:  8  -> 20  MiB/sec
- 4️⃣🟩: 20  -> 50  MiB/sec
- 5️⃣🟪: 50 MiB/sec or more

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
