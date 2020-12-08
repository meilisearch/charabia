mod default_run;
mod initialization;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    default_run::criterion_benchmark(c, &DATA_SET);
    initialization::criterion_benchmark(c, &DATA_SET_SHORT);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

static DATA_SET: [(&str, &str); 6] = [
    ("CHSP1", "人人生而自由﹐在尊严和权利上一律平等。他们赋有理性和良心﹐并应以兄弟关系的精神互相对待。"),
    ("CHSP2", "距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 "),
    ("CHTR1", "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。"),
    ("LTEN1", "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!"),
    ("LTEN2", "The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] "),
    ("LTFR1", "La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain."),
];

static DATA_SET_SHORT: [(&str, &str); 5] = [
    ("SHORT_CH1", "严和"),
    ("SHORT_CH2", "嚴和"),
    ("SHORT_LT1", "City"),
    ("SHORT_LT2", "3°F"),
    ("SHORT_LT3", "île"),
];
