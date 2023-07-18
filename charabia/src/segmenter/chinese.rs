use jieba_rs::Jieba;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::segmenter::Segmenter;

/// Chinese Script specialized [`Segmenter`].
///
/// This Segmenter uses [`Jieba`] internally to segment the provided text
/// without HMM feature.
pub struct ChineseSegmenter;

impl Segmenter for ChineseSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let segmented = JIEBA.cut(to_segment, false); // disable Hidden Markov Models.

        Box::new(segmented.into_iter())
    }
}

fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>,
{
    let path = filename.as_ref();
    if !path.exists() {
        println!("****");
        return vec![];
    }

    if let Ok(file) = File::open(&path) {
        let reader = io::BufReader::new(file);
        let mut lines = Vec::new();
    
        for line in reader.lines() {
            if let Ok(line) = line {
                lines.push(line);
            }
        }
    
        return lines;
    }
    return vec![]
}


static JIEBA: Lazy<Jieba> = Lazy::new(|| {
    let mut jieba = Jieba::new();
    let lines = read_lines("./charabia/dictionaries/chinese_company_names/words.txt");
    for line in lines {
        jieba.add_word(line.as_str(), Some(99 as usize), None);
    }
    jieba
});

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    // Original version of the text.
    const TEXT: &str =
        "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。";

    // Segmented version of the text.
    const SEGMENTED: &[&str] = &[
        "人人",
        "生而自由",
        "﹐",
        "在",
        "尊",
        "嚴",
        "和",
        "權",
        "利",
        "上",
        "一律平等",
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
    ];

    // Segmented and normalized version of the text.
    const TOKENIZED: &[&str] = &[
        "rénrén",
        "shēngérzìyóu",
        ",",
        "zài",
        "zūn",
        "yán",
        "hé",
        "quán",
        "lì",
        "shàng",
        "yīlǜpíngděng",
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
    ];

    // Macro that run several tests on the Segmenter.
    test_segmenter!(ChineseSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Cj, Language::Cmn);
}
