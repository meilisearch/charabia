use criterion::{black_box, BenchmarkId, Criterion};
use meilisearch_tokenizer::{Analyzer, AnalyzerConfig, Token};
use fst::Set;

pub fn criterion_benchmark(c: &mut Criterion, data_set: &[(&str, &str)]) {
    let stop_words = Set::default();
    let analyzer = Analyzer::new(AnalyzerConfig::default_with_stopwords(&stop_words));
    let orig = "人人生而自由﹐在尊严和权利上一律平等。他们赋有理性和良心﹐并应以兄弟关系的精神互相对待。";
    analyzer.analyze(orig);

    let mut group = c.benchmark_group("default-run");

    for &(name, text) in data_set {
        group.bench_with_input(BenchmarkId::new("default-run", name), &(&analyzer, black_box(text)), |b, &(a, s)| b.iter(|| default_run(a, s)));
    }

    group.finish();
}

fn default_run(analyzer: &Analyzer<Vec<u8>>, text: &str) {

    let analyzed = analyzer.analyze(text);
    
    black_box::<Vec<Token>>(analyzed.tokens().collect());
}
