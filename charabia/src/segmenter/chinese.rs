use jieba_rs::Jieba;
use once_cell::sync::Lazy;

use crate::segmenter::Segmenter;

/// Chinese Script specialized [`Segmenter`].
///
/// This Segmenter uses [`Jieba`] internally to segment the provided text
/// without HMM feature.
pub struct ChineseSegmenter;

fn next_gram<const N: usize>(s: &str) -> Option<&str> {
    let mut char_count = 0;
    for (i, _) in s.char_indices() {
        char_count += 1;
        if char_count > N {
            return Some(&s[0..i]);
        }
    }
    if char_count == N {
        return Some(s);
    }
    None
}

fn cut_for_search(s: &str) -> Vec<&str> {
    if s.chars().count() <= 2 {
        return vec![s];
    }
    let mut subwords = Vec::new();
    let mut index = 0;
    loop {
        if let Some(bigram) = next_gram::<2>(&s[index..]).filter(|sub| JIEBA.has_word(sub)) {
            // valid bigram, register it and advance by two characters.
            // greedy thinking: do bigram first, maybe we can get more words
            index += bigram.len();
            subwords.push(bigram);
        } else if let Some(trigram) = next_gram::<3>(&s[index..]).filter(|sub| JIEBA.has_word(sub))
        {
            // valid trigram, register it and advance by three characters.
            index += trigram.len();
            subwords.push(trigram);
        } else if let Some(c) = s[index..].chars().next() {
            //Register the character and advance by one character.
            subwords.push(&s[index..][..c.len_utf8()]);
            index += c.len_utf8();
        } else {
            // no more character, stop.
            break;
        }
    }
    subwords
}

impl Segmenter for ChineseSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let segmented: Vec<&str> = JIEBA
            .cut(to_segment, false) // disable Hidden Markov Models.
            .into_iter()
            .map(|x| cut_for_search(x))
            .flatten()
            .collect();
        Box::new(segmented.into_iter())
    }
}

static JIEBA: Lazy<Jieba> = Lazy::new(Jieba::new);

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    // Original version of the text.
    const TEXT: &str =
        "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待 123 456。";

    // Segmented version of the text.
    const SEGMENTED: &[&str] = &[
        "人人", "生", "而", "自由", "﹐", "在", "尊", "嚴", "和", "權", "利", "上", "一律", "平等",
        "。", "他", "們", "賦", "有", "理性", "和", "良心", "﹐", "並", "應", "以", "兄弟", "關",
        "係", "的", "精神", "互相", "對", "待", " ", "123", " ", "456", "。",
    ];

    // Segmented and normalized version of the text.
    #[cfg(feature = "chinese-normalization-pinyin")]
    const TOKENIZED: &[&str] = &[
        "rénrén",
        "shēng",
        "ér",
        "zìyóu",
        ",",
        "zài",
        "zūn",
        "yán",
        "hé",
        "quán",
        "lì",
        "shàng",
        "yīlǜ",
        "píngděng",
        "。",
        "tā",
        "men",
        "fù",
        "yǒu",
        "lǐxìng",
        "hé",
        "liángxīn",
        ",",
        "bìng",
        "yīng",
        "yǐ",
        "xiōngdì",
        "guān",
        "xì",
        "de",
        "jīngshén",
        "hùxiāng",
        "duì",
        "dài",
        " ",
        "123",
        " ",
        "456",
        "。",
    ];

    #[cfg(not(feature = "chinese-normalization-pinyin"))]
    const TOKENIZED: &[&str] = &[
        "人人", "生", "而", "自由", ",", "在", "尊", "嚴", "和", "權", "利", "上", "一律", "平等",
        "。", "他", "們", "賦", "有", "理性", "和", "良心", ",", "並", "應", "以", "兄弟", "關",
        "係", "的", "精神", "互相", "對", "待", " ", "123", " ", "456", "。",
    ];

    // Macro that run several tests on the Segmenter.
    test_segmenter!(ChineseSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Cj, Language::Cmn);
}
