use charabia::{Language, Script, Segment, Tokenize};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

static DATA_SET: &[((usize, Script, Language), &str)] = &[
    // short texts (~130 bytes)
    ((132, Script::Cj, Language::Cmn), "人人生而自由﹐在尊严和权利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。"),
    ((132, Script::Cj, Language::Jpn), "詳しくは以下の をご覧下さい。語学ないし文学の立場からの価値判断は一切おこなっていません"),
    ((132, Script::Latin, Language::Eng), "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F! Hello guys, my purpose is to benchmark tokenizer properly."),
    ((132, Script::Latin, Language::Fra), "La ville avait d'abord été nommée « Lutèce » ou « boueuse », ici une tentative d'explication par le latin lŭtum « boue »."),
    ((132, Script::Hebrew, Language::Heb), "הַשּׁוּעָל הַמָּהִיר (״הַחוּם״) לֹא יָכוֹל לִקְפֹּץ 8.94 מֶטְרִים, נָכוֹן?"),
    ((132, Script::Thai, Language::Tha), "ไก่จิกเด็กตายเด็กตายบนปากโอ่งไก่อะไรวะโหดจัง"),
    // long texts (~365 bytes)
    ((363, Script::Cj, Language::Cmn), "距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。"),
    ((364, Script::Cj, Language::Jpn), "詳しくは以下の をご覧下さい。語学ないし文学の立場からの価値判断は一切おこなっていません。だけど、バラ科の仲間ということでは「すもももももももものうち」は正しいことになります。すももものうち！今日は「すもももももももものうち」について考えます。"),
    ((363, Script::Latin, Language::Eng), "The City of London Corporation is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownership beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London)."),
    ((363, Script::Latin, Language::Fra), "La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain. Le mot Lutèce resulte de la francisation de Lutetia."),
    ((365, Script::Hebrew, Language::Heb), "הַשּׁוּעָל הַמָּהִיר (״הַחוּם״) לֹא יָכוֹל לִקְפֹּץ 8.94 מֶטְרִים, נָכוֹן? תַּכְלֶס, אִם הוּא הָיָה יָכוֹל, הוּא חֲתִיכַת שׁוּעָל הַשּׁוּעָל הַזֶּה.. אֲבָל הַאִם לֹא כֻּלָּנוּ שׁוּעָלִים בְּעֶצֶם? יתכן."),
    ((366, Script::Thai, Language::Tha), "เราจะทำตามสัญญาขอเวลาอีกไม่นานแล้วแผ่นดินที่งดงามจะคืนกลับมาเราจะทำอย่างซื่อตรงขอแค่เธอจงไว้ใจและศรัทธาแผ่นดินจะดีในไม่ช้า"),
    ];

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn criterion_benchmark(c: &mut Criterion) {
    // tokenize a first time each text to trigger lazy initializations
    for (_name, text) in DATA_SET {
        text.tokenize().count();
    }

    benchmark_texts!(c, segment);
    benchmark_texts!(c, tokenize);
}

fn tokenize(text: &str) {
    black_box(text).tokenize().count();
}

fn segment(text: &str) {
    black_box(text).segment().count();
}

macro_rules! benchmark_texts {
    ($c:expr, $func:ident) => {
        let mut group = $c.benchmark_group(stringify!($func));

        for ((size, script, lang), text) in DATA_SET {
            group.bench_with_input(
                BenchmarkId::new(size.to_string(), format!("{:?}/{:?}", script, lang)),
                &text,
                |b, text| b.iter(|| $func(text)),
            );
        }

        group.finish();
    };
}

use benchmark_texts;
