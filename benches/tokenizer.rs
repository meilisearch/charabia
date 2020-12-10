use std::collections::HashMap;

use criterion::{BenchmarkId, Criterion, black_box};
use fst::Set;

use meilisearch_tokenizer::{Analyzer, AnalyzerConfig};
use meilisearch_tokenizer::analyzer::{Language, Pipeline, Script};
use meilisearch_tokenizer::tokenizer::{LegacyMeilisearch, Tokenizer, Jieba, UnicodeSegmenter};


fn init_analyzer_with_tokenizer<'a>(tokenizer: impl Tokenizer + 'static, stop_words: &'a Set<Vec<u8>>) -> Analyzer<'a, Vec<u8>> {
    let mut pipeline_map: HashMap<(Script, Language), Pipeline> = HashMap::new();
    pipeline_map.insert((Script::Other, Language::Other), Pipeline::default()
        .set_tokenizer(tokenizer));

    let analyzer = Analyzer::new(AnalyzerConfig::new(pipeline_map, stop_words));
    
    // analyze a first time to trigger lazy initializations
    analyzer.analyze("Hello");

    analyzer
}

pub fn criterion_benchmark(c: &mut Criterion, data_set: &[(&str, &str)]) {
    let stop_words = Set::default();

    let mut group = c.benchmark_group("tokenizer");

    let analyzer = init_analyzer_with_tokenizer(LegacyMeilisearch, &stop_words);
    for &(name, text) in data_set {
        group.bench_function(BenchmarkId::new("LegacyMeilisearch", name), |b| b.iter(|| run(&analyzer, black_box(text))));
    }

    let analyzer = init_analyzer_with_tokenizer(UnicodeSegmenter, &stop_words);
    for &(name, text) in data_set {
        group.bench_function(BenchmarkId::new("UnicodeSegmenter", name), |b| b.iter(|| run(&analyzer, black_box(text))));
    }

    let analyzer = init_analyzer_with_tokenizer(Jieba, &stop_words);
    for &(name, text) in data_set {
        group.bench_function(BenchmarkId::new("Jieba", name), |b| b.iter(|| run(&analyzer, black_box(text))));
    }

    group.finish();
}

fn run(analyzer: &Analyzer<Vec<u8>>, text: &str) {
    let analyzed = analyzer.analyze(text);
    
    black_box(analyzed.tokens().for_each(|_|{}));
}
