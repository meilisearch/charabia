use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use meilisearch_tokenizer::{Normalize, Segment, Tokenize};

static DATA_SET: [((&str, &str), &str); 11] = [
    // short texts (~130 bytes)
    (("132B", "CHSP"), "人人生而自由﹐在尊严和权利上一律平等。他们赋有理性和良心﹐并应以兄弟关系的精神互相对待。"),
    (("132B", "CHTR"), "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。"),
    (("132B", "CHJP"), "詳しくは以下の をご覧下さい。語学ないし文学の立場からの価値判断は一切おこなっていません"),
    (("132B", "HIJP"), "だけど、バラ科の仲間ということでは「すもももももももものうち」は正しいことになります。。"),
    (("132B", "LTEN"), "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F! Hello guys, my purpose is to benchmark tokenizer properly."),
    (("132B", "LTFR"), "La ville avait d'abord été nommée « Lutèce » ou « boueuse », ici une tentative d'explication par le latin lŭtum « boue »."),

    // long texts (~365 bytes)
    (("363B", "CHSP"), "距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。"),
    (("364B", "CHJP"), "本サイトで可能な検索は文字列検索（全文検索）だけですが、形態論情報を利用した検索サイト「中納言」も開設しています。詳しくは以下の URL をご覧下さい。なお、中納言の利用には申請が必要です。語学ないし文学の立場からの価値判断は一切おこなっていません"),
    (("364B", "HIJP"), "すもももももももものうち！「すもも」も「もも」も丸々としておいしそうという特徴で見ると「すもももももももものうち」は正しいことになります。今日は「すもももももももものうち」について考えます。今日は「すもももももももものうち」について考えます。"),
    (("363B", "LTEN"), "The City of London Corporation is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownership beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London)."),
    (("363B", "LTFR"), "La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain. Le mot Lutèce resulte de la francisation de Lutetia."),
];

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn criterion_benchmark(c: &mut Criterion) {
    // tokenize a first time each text to trigger lazy initializations
    for (_name, text) in DATA_SET {
        text.tokenize().count();
    }

    benchmark_texts!(c, segment);
    benchmark_texts!(c, segment_normalize);
    benchmark_texts!(c, tokenize);
}

fn tokenize(text: &str) {
    black_box(text).tokenize().count();
}

fn segment_normalize(text: &str) {
    black_box(text).segment().normalize().count();
}

fn segment(text: &str) {
    black_box(text).segment().count();
}

macro_rules! benchmark_texts {
    ($c:expr, $func:ident) => {
        let mut group = $c.benchmark_group(stringify!($func));

        for ((size, script_lang), text) in DATA_SET {
            group.bench_with_input(BenchmarkId::new(size, script_lang), &text, |b, text| {
                b.iter(|| $func(text))
            });
        }

        group.finish();
    };
}

use benchmark_texts;
