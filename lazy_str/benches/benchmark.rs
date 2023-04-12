use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazy_str::LazyStr;

fn lazy_ipsum() {
    let s = LazyStr::new("");
    let s = s + "Lorem" + "ipsum" + "dolor" + "sit" + "amet" + "consectetur" + "adipiscing" +
        "elit" + "sed" + "do" + "eiusmod" + "tempor" + "incididunt" + "ut" + "labore" + "et" +
        "dolore" + "magna" + "aliqua" + "Ut" + "enim" + "ad" + "minim" + "veniam" + "quis" +
        "nostrud" + "exercitation" + "ullamco" + "laboris" + "nisi" + "ut" + "aliquip" + "ex" +
        "ea" + "commodo" + "consequat" + "Duis" + "aute" + "irure" + "dolor" + "in" +
        "reprehenderit" + "in" + "voluptate" + "velit" + "esse" + "cillum" + "dolore" + "eu" +
        "fugiat" + "nulla" + "pariatur" + "Excepteur" + "sint" + "occaecat" + "cupidatat" + "non" +
        "proident" + "sunt" + "in" + "culpa" + "qui" + "officia" + "deserunt" + "mollit" + "anim" +
        "id" + "est" + "laborum";
    let _ = s.to_string();
}

fn normal_ipsum() {

}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ipsum_benach", |b| b.iter(|| lazy_ipsum()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);



