use std::collections::HashMap;

use criterion::{black_box, BenchmarkId, Criterion};
use fst::Set;
use meilisearch_tokenizer::analyzer::{Language, Pipeline, Script};
use meilisearch_tokenizer::normalizer::{
    DeunicodeNormalizer, IdentityNormalizer, LowercaseNormalizer, Normalizer,
};
use meilisearch_tokenizer::tokenizer::LegacyMeilisearch;
use meilisearch_tokenizer::{Analyzer, AnalyzerConfig};

fn init_analyzer_with_normalizer<'a>(
    normalizer: impl Normalizer + 'static,
    stop_words: &'a Set<Vec<u8>>,
) -> Analyzer<'a, Vec<u8>> {
    let mut pipeline_map: HashMap<(Script, Language), Pipeline> = HashMap::new();
    pipeline_map.insert(
        (Script::Other, Language::Other),
        Pipeline::default().set_tokenizer(LegacyMeilisearch).set_normalizer(normalizer),
    );

    let analyzer = Analyzer::new(AnalyzerConfig::new(pipeline_map, stop_words));

    // analyze a first time to trigger lazy initializations
    analyzer.analyze("Hello");

    analyzer
}

pub fn criterion_benchmark(c: &mut Criterion, data_set: &[(&str, &str)]) {
    let stop_words = Set::default();

    let mut group = c.benchmark_group("normalizer");

    let analyzer = init_analyzer_with_normalizer(IdentityNormalizer, &stop_words);
    for &(name, text) in data_set {
        group.bench_with_input(
            BenchmarkId::new("IdentityNormalizer", name),
            &(&analyzer, text),
            |b, &(a, s)| b.iter(|| run(a, s)),
        );
    }

    let analyzer = init_analyzer_with_normalizer(DeunicodeNormalizer::default(), &stop_words);
    for &(name, text) in data_set {
        group.bench_with_input(
            BenchmarkId::new("DeunicodeNormalizer", name),
            &(&analyzer, text),
            |b, &(a, s)| b.iter(|| run(a, s)),
        );
    }

    let analyzer = init_analyzer_with_normalizer(LowercaseNormalizer, &stop_words);
    for &(name, text) in data_set {
        group.bench_with_input(
            BenchmarkId::new("LowercaseNormalizer", name),
            &(&analyzer, text),
            |b, &(a, s)| b.iter(|| run(a, s)),
        );
    }

    group.finish();
}

fn run(analyzer: &Analyzer<Vec<u8>>, text: &str) {
    let analyzed = analyzer.analyze(text);

    black_box(analyzed.tokens().for_each(|_| {}));
}
