// bench.rs
#![feature(test)]


use whatlang::{Lang, Script};
extern crate test;

use meilisearch_tokenizer::*;
use test::Bencher;

enum LangDetection {
    Forced(Lang),
    AutoWithDefault(Lang),
    Auto,
    None,
}

#[derive(PartialEq, Debug)]
enum Precision {
    Hight,
    Low,
}

struct TokenizerBuilder;

impl TokenizerBuilder {
    fn new() -> TokenizerBuilder { TokenizerBuilder }
    fn lang_detection(&self, l: LangDetection) {}
    fn precision(&self, p: Precision) {}
    fn keep_ponctuation(&self, active: bool) {}
    fn lowercased(&self, active: bool) {}
    fn default_stopwords(&self, active: bool) {}
    fn unicode(&self, active: bool) {}
    fn build<'a>(&self, s: &'a str) -> Tokenizer<'a> { Tokenizer::new(s) }
}

#[bench]
fn bench_fra_lang_auto(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_fra_lang_auto_impact_ponctualtion(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(true);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_fra_lang_auto_impact_lowercased(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(true);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_fra_lang_auto_impact_unicode(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(true);

        let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_fra_lang_auto_full_hight(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Hight);
        builder.keep_ponctuation(true);
        builder.lowercased(true);
        builder.default_stopwords(true);

        let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_fra_lang_auto_full_low(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(true);
        builder.lowercased(true);
        builder.default_stopwords(true);

        let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_fra_lang_fra(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Fra));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_fra_lang_fra_impact_ponctualtion(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Fra));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(true);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_fra_lang_fra_impact_lowercased(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Fra));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(true);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_fra_lang_fra_impact_unicode(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Fra));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(true);

        let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_fra_lang_fra_full_hight(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Fra));
        builder.precision(Precision::Hight);
        builder.keep_ponctuation(true);
        builder.lowercased(true);
        builder.default_stopwords(true);

        let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_fra_lang_fra_full_low(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Fra));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(true);
        builder.lowercased(true);
        builder.default_stopwords(true);

        let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
        for _ in tokens{
            continue
        }
    });
}

//Anglais

#[bench]
fn bench_eng_lang_auto(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_eng_lang_auto_impact_ponctualtion(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(true);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_eng_lang_auto_impact_lowercased(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(true);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_eng_lang_auto_impact_unicode(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(true);

        let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_eng_lang_auto_full_hight(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Hight);
        builder.keep_ponctuation(true);
        builder.lowercased(true);
        builder.default_stopwords(true);

        let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_eng_lang_auto_full_low(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(true);
        builder.lowercased(true);
        builder.default_stopwords(true);

        let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_eng_lang_eng(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Eng));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_eng_lang_eng_impact_ponctualtion(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Eng));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(true);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_eng_lang_eng_impact_lowercased(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Eng));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(true);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_eng_lang_eng_impact_unicode(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Eng));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(true);

        let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_eng_lang_eng_full_hight(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Eng));
        builder.precision(Precision::Hight);
        builder.keep_ponctuation(true);
        builder.lowercased(true);
        builder.default_stopwords(true);

        let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_eng_lang_eng_full_low(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Eng));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(true);
        builder.lowercased(true);
        builder.default_stopwords(true);

        let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
        for _ in tokens{
            continue
        }
    });
}

// Chinois

#[bench]
fn bench_cnn_lang_auto(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_cnn_lang_auto_impact_ponctualtion(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(true);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_cnn_lang_auto_impact_lowercased(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(true);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_cnn_lang_auto_impact_unicode(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(true);

        let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_cnn_lang_auto_full_hight(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Hight);
        builder.keep_ponctuation(true);
        builder.lowercased(true);
        builder.default_stopwords(true);

        let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_cnn_lang_auto_full_low(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(true);
        builder.lowercased(true);
        builder.default_stopwords(true);

        let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_cnn_lang_cnn(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Fra));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_cnn_lang_cnn_impact_ponctualtion(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Fra));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(true);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_cnn_lang_cnn_impact_lowercased(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Fra));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(true);
        builder.default_stopwords(false);
        builder.unicode(false);

        let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_cnn_lang_cnn_impact_unicode(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Fra));
        builder.precision(Precision::Low);
        builder.keep_ponctuation(false);
        builder.lowercased(false);
        builder.default_stopwords(false);
        builder.unicode(true);

        let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_cnn_lang_cnn_full_hight(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Forced(Lang::Fra));
        builder.precision(Precision::Hight);
        builder.keep_ponctuation(true);
        builder.lowercased(true);
        builder.default_stopwords(true);

        let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
        for _ in tokens{
            continue
        }
    });
}

#[bench]
fn bench_cnn_lang_cnn_full_low(b: &mut Bencher) {
    b.iter(|| {
        let mut builder = TokenizerBuilder::new();
        builder.lang_detection(LangDetection::Auto);
        builder.precision(Precision::Low);
        builder.keep_ponctuation(true);
        builder.lowercased(true);
        builder.default_stopwords(true);

        let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
        for _ in tokens{
            continue
        }
    });
}
