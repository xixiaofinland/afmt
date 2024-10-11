use afmt::{config::Session, format};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let sample_path = "../large_apex_files/files/MetadataService2.cls";
    let session = Session::create_session_from_config(None, vec![sample_path.to_string()])
        .expect("Failed to create session");

    c.bench_function("format_apex", |b| {
        b.iter(|| {
            let _ = format(black_box(session.clone()));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
