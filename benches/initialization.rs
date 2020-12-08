use std::collections::HashMap;

use criterion::{BenchmarkId, Criterion, black_box};
use meilisearch_tokenizer::{Analyzer, AnalyzerConfig, analyzer::{Language, Pipeline, Script}, normalizer::DeunicodeNormalizer, normalizer::LowercaseNormalizer, normalizer::Normalizer, processors::ChineseTranslationPreProcessor, tokenizer::{Jieba, LegacyMeilisearch}, detection::is_cjk};
use fst::Set;

pub fn criterion_benchmark(c: &mut Criterion, data_set: &[(&str, &str)]) {
    let mut group = c.benchmark_group("initialization");

    for &(name, text) in data_set {
        group.bench_with_input(BenchmarkId::new("default", name), black_box(text), |b, s| b.iter(|| default_init(s)));
    }

    for &(name, text) in data_set {
        group.bench_with_input(BenchmarkId::new("pre:identity-tok:legacy-nor:deunicode+lowercase", name), black_box(text), |b, s| b.iter(|| legacy_tokenizer_deunicode_lowercase_normalizer(s)));
    }

    for &(name, text) in data_set {
        group.bench_with_input(BenchmarkId::new("pre:translate-tok:jieba-nor:deunicode+lowercase", name), black_box(text), |b, s| b.iter(|| translation_pre_jieba_tok_deunicode_lowercase_norm(s)));
    }

    group.finish();
}

fn default_init(text: &str) {
    let stop_words = Set::default();
    let analyzer = Analyzer::new(AnalyzerConfig::default_with_stopwords(&stop_words));

    analyzer.analyze(text);
}

fn legacy_tokenizer_deunicode_lowercase_normalizer(text: &str) {
    let mut pipeline_map: HashMap<(Script, Language), Pipeline> = HashMap::new();
    let latin_normalizer: Vec<Box<dyn Normalizer>> = vec![Box::new(DeunicodeNormalizer::default()), Box::new(LowercaseNormalizer)];
    pipeline_map.insert((Script::Latin, Language::Other), Pipeline::default()
        .set_tokenizer(LegacyMeilisearch)
        .set_normalizer(latin_normalizer));

    let stop_words = Set::default();
    let analyzer = Analyzer::new(AnalyzerConfig::new(pipeline_map, &stop_words));

    analyzer.analyze(text);
}

fn translation_pre_jieba_tok_deunicode_lowercase_norm(text: &str) {
    let mut pipeline_map: HashMap<(Script, Language), Pipeline> = HashMap::new();
    let chinese_deunicoder = DeunicodeNormalizer::new(&|text: &str| text.chars().next().map_or(false, is_cjk));
    let chinese_normalizer: Vec<Box<dyn Normalizer>> = vec![Box::new(chinese_deunicoder), Box::new(LowercaseNormalizer)];
    pipeline_map.insert((Script::Mandarin, Language::Other), Pipeline::default()
        .set_pre_processor(ChineseTranslationPreProcessor)
        .set_tokenizer(Jieba::default())
        .set_normalizer(chinese_normalizer));

    let stop_words = Set::default();
    let analyzer = Analyzer::new(AnalyzerConfig::new(pipeline_map, &stop_words));

    analyzer.analyze(text);
}
