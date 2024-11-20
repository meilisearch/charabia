use charabia::{Language, Script, Segment, Tokenize};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

static DATA_SET: &[((usize, Script, Language), &str)] = &[
    // short texts (~130 bytes)
    ((132, Script::Cj, Language::Cmn), "人人生而自由﹐在尊严和权利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。"),
    ((132, Script::Cj, Language::Jpn), "詳しくは以下の をご覧下さい。語学ないし文学の立場からの価値判断は一切おこなっていません"),
    ((132, Script::Latin, Language::Eng), "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F! Hello guys, my purpose is to benchmark tokenizer properly."),
    ((132, Script::Latin, Language::Fra), "La ville avait d'abord été nommée « Lutèce » ou « boueuse », ici une tentative d'explication par le latin lŭtum « boue »."),
    ((132, Script::Hebrew, Language::Heb), "הַשּׁוּעָל הַמָּהִיר (״הַחוּם״) לֹא יָכוֹל לִקְפֹּץ 8.94 מֶטְרִים, נָכוֹן?"),
    ((132, Script::Thai, Language::Tha), "ไก่จิกเด็กตายเด็กตายบนปากโอ่งไก่อะไรวะโหดจัง"),
    ((132, Script::Hangul, Language::Kor), "제119조 ① 대한민국의 경제질서는 개인과 기업의 경제상의 자유와 창의를 존중함을 기본으로 한다."),
    ((130, Script::Greek, Language::Ell), "Οι θερμοκρασίες είναι σπάνια υπερβολικές στις παραθαλάσσιες περιοχές."),
    ((132, Script::Khmer, Language::Khm), "ធ្វេីមនុស្សត្រូវចេះស្រលាញ់នឹងជួយគ្នាទៅវិញទៅមក ព្រោះពិភពលោកនេះមានទុកច្រេីនហេីយគួយតែមានអំពេីល្អច្រេីនមិនថាជួយបាន១រឺ២នាក់ច្រេីនរឺតិចទេ៕"),
    ((132, Script::Arabic, Language::Ara), "اللُّغَةُ العربية هي أكثر اللغات السامية تحدثا، ومن أكثر اللغات انتشارا"),
    ((134, Script::Arabic, Language::Vie), "Các nhà nước trong lịch sử Việt Nam có những quốc hiệu khác nhau như Xích Quỷ, Văn Lang, Đại Việt, Đại"),
    ((131, Script::Latin, Language::Deu), "Deutschland vereint Alpen, Küsten und Städte wie Berlin. Kultur und Geschichte prägen das Land, das Natur und Moderne verbindet."),

    // long texts (~365 bytes)
    ((363, Script::Cj, Language::Cmn), "距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。"),
    ((364, Script::Cj, Language::Jpn), "詳しくは以下の をご覧下さい。語学ないし文学の立場からの価値判断は一切おこなっていません。だけど、バラ科の仲間ということでは「すもももももももものうち」は正しいことになります。すももものうち！今日は「すもももももももものうち」について考えます。"),
    ((363, Script::Latin, Language::Eng), "The City of London Corporation is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownership beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London)."),
    ((363, Script::Latin, Language::Fra), "La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain. Le mot Lutèce resulte de la francisation de Lutetia."),
    ((365, Script::Hebrew, Language::Heb), "הַשּׁוּעָל הַמָּהִיר (״הַחוּם״) לֹא יָכוֹל לִקְפֹּץ 8.94 מֶטְרִים, נָכוֹן? תַּכְלֶס, אִם הוּא הָיָה יָכוֹל, הוּא חֲתִיכַת שׁוּעָל הַשּׁוּעָל הַזֶּה.. אֲבָל הַאִם לֹא כֻּלָּנוּ שׁוּעָלִים בְּעֶצֶם? יתכן."),
    ((366, Script::Thai, Language::Tha), "เราจะทำตามสัญญาขอเวลาอีกไม่นานแล้วแผ่นดินที่งดงามจะคืนกลับมาเราจะทำอย่างซื่อตรงขอแค่เธอจงไว้ใจและศรัทธาแผ่นดินจะดีในไม่ช้า"),
    ((364, Script::Hangul, Language::Kor), "제30조 타인의 범죄행위로 인하여 생명·신체에 대한 피해를 받은 국민은 법률이 정하는 바에 의하여 국가로부터 구조를 받을 수 있다. ② 명령·규칙 또는 처분이 헌법이나 법률에 위반되는 여부가 재판의 전제가 된 경우에는 대법원은 이를 최종적으로 심사할 권한을 가진다."),
    ((364, Script::Greek, Language::Ell), "Η άνοιξη έχει μικρή διάρκεια, διότι ο μεν χειμώνας είναι όψιμος, το δε καλοκαίρι αρχίζει πρώιμα. Το φθινόπωρο είναι μακρύ και θερμό και πολλές φορές παρατείνεται στη νότια Ελλάδα και τα νησιά μέχρι τα"),
    ((327, Script::Khmer, Language::Khm), "រឿងពីរដែលមនុស្សហាមចិត្តខ្លួនឯងមិនបានគឺ សើច និង ស្រឡាញ់។ តែសម្រាប់ខ្ញុំ ប្រាក់ ចន្ទធីតា រឿងមួយទៀតដែលខ្ញុំហាមចិត្តខ្លួនឯងមិនបាននោះ គឺញ៉ាំ គេគ្រប់គ្នាពេលខូចចិត្តបាយទឹកមិនបានទេ តែខ្ញុំពេលខូចចិត្តដឹងតែឃ្លាន ញ៉ាំច្រើនឬតិចក៏អាស្រ័យលើថាទំហំនៃការខូចចិត្តខ្លាំងឬខ្សោយ។"),
    ((366, Script::Arabic, Language::Ara), "العربية لغةٌ رسمية في كل دول الوطن العربي (إضافة إلى كونها لغة رسمية في تشاد وإريتريا). وهي إحدى اللغات الرسمية الست في منظمة الأمم المتحدة، ويُحتفل بالعربية في 18 ديسمبر كذكرى اعتمادها في الأمم المتحدة."),
    ((365, Script::Latin, Language::Vie), "Lãnh thổ Việt Nam xuất hiện con người sinh sống từ thời đại đồ đá cũ, khởi đầu với các nhà nước Văn Lang, Âu Lạc. Âu Lạc bị nhà Triệu ở phương Bắc thôn tính vào đầu thế kỷ thứ 2 TCN sau đó là thời kỳ Bắc thuộc kéo dài hơn một thiên niên kỷ.Chế độ quân chủ độc lập"),
    ((354, Script::Latin, Language::Deu), "Magdeburg, die Hauptstadt Sachsen-Anhalts, beeindruckt mit dem Magdeburger Dom, dem Jahrtausendturm im Elbauenpark und dem Wasserstraßenkreuz. Der Domplatz ist umgeben von Bauwerken, wie dem Hundertwasserhaus. Der Elbauenpark bietet viele Freizeitmöglichkeiten, während die Magdeburger Börde für fruchtbare Ackerflächen für z.B. Zuckerrüben bekannt ist."),
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
