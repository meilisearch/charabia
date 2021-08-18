use std::collections::HashMap;

use criterion::{BenchmarkId, Criterion};
use fst::Set;
use meilisearch_tokenizer::analyzer::{Language, Pipeline, Script};
use meilisearch_tokenizer::detection::is_cjk;
use meilisearch_tokenizer::normalizer::{DeunicodeNormalizer, LowercaseNormalizer, Normalizer};
use meilisearch_tokenizer::processors::ChineseTranslationPreProcessor;
use meilisearch_tokenizer::tokenizer::{Jieba, LegacyMeilisearch};
use meilisearch_tokenizer::{Analyzer, AnalyzerConfig};

pub fn criterion_benchmark(c: &mut Criterion, data_set: &[(&str, &str)]) {
    let mut group = c.benchmark_group("initialization");

    for &(name, text) in data_set {
        group.bench_with_input(BenchmarkId::new("default", name), text, |b, s| {
            b.iter(|| default_init(s))
        });
    }

    for &(name, text) in data_set {
        group.bench_with_input(
            BenchmarkId::new("pre:identity-tok:legacy-nor:deunicode+lowercase", name),
            text,
            |b, s| b.iter(|| legacy_tok_deunicode_lowercase_norm(s)),
        );
    }

    for &(name, text) in data_set {
        group.bench_with_input(
            BenchmarkId::new("pre:translate-tok:jieba-nor:deunicode+lowercase", name),
            text,
            |b, s| b.iter(|| translation_pre_jieba_tok_deunicode_lowercase_norm(s)),
        );
    }

    group.finish();
}

fn default_init(text: &str) {
    let stop_words = Set::default();
    let analyzer = Analyzer::new(AnalyzerConfig::default_with_stopwords(&stop_words));

    analyzer.analyze(text);
}

fn legacy_tok_deunicode_lowercase_norm(text: &str) {
    let mut pipeline_map: HashMap<(Script, Language), Pipeline> = HashMap::new();
    let latin_normalizer: Vec<Box<dyn Normalizer>> =
        vec![Box::new(DeunicodeNormalizer::default()), Box::new(LowercaseNormalizer)];
    pipeline_map.insert(
        (Script::Other, Language::Other),
        Pipeline::default().set_tokenizer(LegacyMeilisearch).set_normalizer(latin_normalizer),
    );

    let stop_words = Set::default();
    let analyzer = Analyzer::new(AnalyzerConfig::new(pipeline_map, &stop_words));

    analyzer.analyze(text);
}

fn translation_pre_jieba_tok_deunicode_lowercase_norm(text: &str) {
    let mut pipeline_map: HashMap<(Script, Language), Pipeline> = HashMap::new();
    let chinese_deunicoder =
        DeunicodeNormalizer::new(&|text: &str| text.chars().next().map_or(false, is_cjk));
    let chinese_normalizer: Vec<Box<dyn Normalizer>> =
        vec![Box::new(chinese_deunicoder), Box::new(LowercaseNormalizer)];
    pipeline_map.insert(
        (Script::Other, Language::Other),
        Pipeline::default()
            .set_pre_processor(ChineseTranslationPreProcessor)
            .set_tokenizer(Jieba::default())
            .set_normalizer(chinese_normalizer),
    );

    let stop_words = Set::default();
    let analyzer = Analyzer::new(AnalyzerConfig::new(pipeline_map, &stop_words));

    analyzer.analyze(text);
}
