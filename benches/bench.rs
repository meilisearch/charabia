mod default_run;
mod initialization;
mod normalizer;
mod tokenizer;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    initialization::criterion_benchmark(c, &DATA_SET_SHORT);
    default_run::criterion_benchmark(c, &DATA_SET);
    tokenizer::criterion_benchmark(c, &DATA_SET);
    normalizer::criterion_benchmark(c, &DATA_SET);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

static DATA_SET: [(&str, &str); 7] = [
    ("132B_CHSP1", "人人生而自由﹐在尊严和权利上一律平等。他们赋有理性和良心﹐并应以兄弟关系的精神互相对待。"),
    ("132B_CHTR1", "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。"),
    ("132B_LTEN1", "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F! Hello guys, my purpose is to benchmark tokenizer properly."),
    ("132B_LTFR1", "La ville avait d'abord été nommée « Lutèce » ou « boueuse », ici une tentative d'explication par le latin lŭtum « boue »."),
    ("363B_CHSP2", "距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。"),
    ("363B_LTEN2", "The City of London Corporation is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownership beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London)."),
    ("363B_LTFR1", "La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain. Le mot Lutèce resulte de la francisation de Lutetia."),
];

static DATA_SET_SHORT: [(&str, &str); 5] = [
    ("SHORT_CH1", "严和"),
    ("SHORT_CH2", "嚴和"),
    ("SHORT_LT1", "City"),
    ("SHORT_LT2", "3°F"),
    ("SHORT_LT3", "île"),
];
