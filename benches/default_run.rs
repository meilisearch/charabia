use criterion::{black_box, BenchmarkId, Criterion};
use fst::Set;
use meilisearch_tokenizer::{Analyzer, AnalyzerConfig};

pub fn criterion_benchmark(c: &mut Criterion, data_set: &[(&str, &str)]) {
    let stop_words = Set::default();
    let analyzer = Analyzer::new(AnalyzerConfig::default_with_stopwords(&stop_words));

    // analyze a first time each text to trigger lazy initializations
    for &(_name, text) in data_set {
        analyzer.analyze(text);
    }

    let mut group = c.benchmark_group("default-run");

    for &(name, text) in data_set {
        group.bench_with_input(
            BenchmarkId::new("default-run", name),
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
