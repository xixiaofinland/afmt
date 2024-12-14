use afmt::{format, formatter::Formatter};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

pub fn criterion_benchmark(c: &mut Criterion) {
    let sample_dir = "samples";
    let apex_files = fs::read_dir(sample_dir)
        .expect("Failed to read samples directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()? == "cls" {
                Some(path.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    // Benchmark each file in a single Criterion report
    for sample_path in apex_files {
        let file_name = sample_path.split('/').last().unwrap_or("Unknown file");

        let session = Formatter::create_from_config(None, vec![sample_path.to_string()])
            .expect("Failed to create session");

        // Add benchmarks for each file to the same report
        c.bench_function(&format!("format_apex_{}", file_name), |b| {
            b.iter(|| {
                let _ = format(black_box(session.clone()));
            })
        });
    }
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .measurement_time(std::time::Duration::new(10, 0))
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = criterion_benchmark
}
criterion_main!(benches);
