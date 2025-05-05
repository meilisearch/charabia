use jieba_rs::Jieba;
use once_cell::sync::Lazy;

use crate::segmenter::Segmenter;

/// Chinese Script specialized [`Segmenter`].
///
/// This Segmenter uses [`Jieba`] internally to segment the provided text
/// without HMM feature.
pub struct ChineseSegmenter;

fn next_gram<const N: usize>(s: &str) -> Option<&str> {
    match s.char_indices().nth(N - 1) {
        Some((byte_index, c)) => Some(&s[0..(byte_index + c.len_utf8())]),
        None => None,
    }
}

fn cut_for_search<'a>(s: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    if s.chars().count() <= 2 {
        return Box::new(std::iter::once(s));
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
        } else if let Some(single) = next_gram::<1>(&s[index..]) {
            //Register the character and advance by one character.
            index += single.len();
            subwords.push(single);
        } else {
            // no more character, stop.
            break;
        }
    }
    Box::new(subwords.into_iter())
}

impl Segmenter for ChineseSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let segmented: Vec<&str> = JIEBA
            .cut(to_segment, false) // disable Hidden Markov Models.
            .into_iter()
            .flat_map(|x| cut_for_search(x))
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
        "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。人民的意志是政府权力的基础，这一意志应以定期的和真正的选举予以表现。夏天，像是哼着小曲的少年，恶作剧般在大地上洒满每一种灿烂的颜色。 123 456。";

    // Segmented version of the text.
    const SEGMENTED: &[&str] = &[
        "人人",
        "生",
        "而",
        "自由",
        "﹐",
        "在",
        "尊",
        "嚴",
        "和",
        "權",
        "利",
        "上",
        "一律",
        "平等",
        "。",
        "他",
        "們",
        "賦",
        "有",
        "理性",
        "和",
        "良心",
        "﹐",
        "並",
        "應",
        "以",
        "兄弟",
        "關",
        "係",
        "的",
        "精神",
        "互相",
        "對",
        "待",
        "。",
        "人民",
        "的",
        "意志",
        "是",
        "政府",
        "权力",
        "的",
        "基础",
        "，",
        "这",
        "一",
        "意志",
        "应",
        "以",
        "定期",
        "的",
        "和",
        "真正",
        "的",
        "选举",
        "予以",
        "表现",
        "。",
        "夏天",
        "，",
        "像是",
        "哼",
        "着",
        "小曲",
        "的",
        "少年",
        "，",
        "恶作剧",
        "般",
        "在",
        "大",
        "地上",
        "洒满",
        "每",
        "一种",
        "灿烂",
        "的",
        "颜色",
        "。",
        " ",
        "123",
        " ",
        "456",
        "。",
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
        "。",
        "rénmín",
        "de",
        "yìzhì",
        "shì",
        "zhèngfǔ",
        "quánlì",
        "de",
        "jīchǔ",
        ",",
        "zhè",
        "yī",
        "yìzhì",
        "yīng",
        "yǐ",
        "dìngqī",
        "de",
        "hé",
        "zhēnzhèng",
        "de",
        "xuǎnjǔ",
        "yǔyǐ",
        "biǎoxiàn",
        "。",
        "xiàtiān",
        ",",
        "xiàngshì",
        "hēng",
        "zhe",
        "xiǎoqū",
        "de",
        "shǎonián",
        ",",
        "èzuòjù",
        "bān",
        "zài",
        "dà",
        "dìshàng",
        "sǎmǎn",
        "měi",
        "yīzhǒng",
        "cànlàn",
        "de",
        "yánsè",
        "。",
        " ",
        "123",
        " ",
        "456",
        "。",
    ];

    #[cfg(not(feature = "chinese-normalization-pinyin"))]
    const TOKENIZED: &[&str] = &[
        "人人",
        "生",
        "而",
        "自由",
        ",",
        "在",
        "尊",
        "嚴",
        "和",
        "權",
        "利",
        "上",
        "一律",
        "平等",
        "。",
        "他",
        "們",
        "賦",
        "有",
        "理性",
        "和",
        "良心",
        ",",
        "並",
        "應",
        "以",
        "兄弟",
        "關",
        "係",
        "的",
        "精神",
        "互相",
        "對",
        "待",
        "。",
        "人民",
        "的",
        "意志",
        "是",
        "政府",
        "权力",
        "的",
        "基礎",
        ",",
        "这",
        "一",
        "意志",
        "應",
        "以",
        "定期",
        "的",
        "和",
        "眞正",
        "的",
        "選舉",
        "予以",
        "表現",
        "。",
        "夏天",
        ",",
        "像是",
        "哼",
        "着",
        "小曲",
        "的",
        "少年",
        ",",
        "惡作劇",
        "般",
        "在",
        "大",
        "地上",
        "洒滿",
        "每",
        "一种",
        "灿爛",
        "的",
        "顏色",
        "。",
        " ",
        "123",
        " ",
        "456",
        "。",
    ];

    // Macro that run several tests on the Segmenter.
    test_segmenter!(ChineseSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Cj, Language::Cmn);
}
