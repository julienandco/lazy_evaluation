use criterion::{criterion_group, criterion_main, Criterion};
use lazy_str::LazyStr;

#[inline(never)]
fn lazy_ipsum() -> String {
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
    s.to_string()
}

#[inline(never)]
fn normal_add_ipsum() -> String {
    let s = String::new();
    let s = s + "Lorem" + "ipsum" + "dolor" + "sit" + "amet" + "consectetur" + "adipiscing" +
        "elit" + "sed" + "do" + "eiusmod" + "tempor" + "incididunt" + "ut" + "labore" + "et" +
        "dolore" + "magna" + "aliqua" + "Ut" + "enim" + "ad" + "minim" + "veniam" + "quis" +
        "nostrud" + "exercitation" + "ullamco" + "laboris" + "nisi" + "ut" + "aliquip" + "ex" +
        "ea" + "commodo" + "consequat" + "Duis" + "aute" + "irure" + "dolor" + "in" +
        "reprehenderit" + "in" + "voluptate" + "velit" + "esse" + "cillum" + "dolore" + "eu" +
        "fugiat" + "nulla" + "pariatur" + "Excepteur" + "sint" + "occaecat" + "cupidatat" + "non" +
        "proident" + "sunt" + "in" + "culpa" + "qui" + "officia" + "deserunt" + "mollit" + "anim" +
        "id" + "est" + "laborum";
    s
}

#[inline(never)]
fn normal_push_ipsum() -> String {
    let mut s = String::new();
    s.push_str("Lorem");
    s.push_str("ipsum");
    s.push_str("dolor");
    s.push_str("sit");
    s.push_str("amet");
    s.push_str("consectetur");
    s.push_str("adipiscing");
    s.push_str("elit");
    s.push_str("sed");
    s.push_str("do");
    s.push_str("eiusmod");
    s.push_str("tempor");
    s.push_str("incididunt");
    s.push_str("ut");
    s.push_str("labore");
    s.push_str("et");
    s.push_str("dolore");
    s.push_str("magna");
    s.push_str("aliqua");
    s.push_str("Ut");
    s.push_str("enim");
    s.push_str("ad");
    s.push_str("minim");
    s.push_str("veniam");
    s.push_str("quis");
    s.push_str("nostrud");
    s.push_str("exercitation");
    s.push_str("ullamco");
    s.push_str("laboris");
    s.push_str("nisi");
    s.push_str("ut");
    s.push_str("aliquip");
    s.push_str("ex");
    s.push_str("ea");
    s.push_str("commodo");
    s.push_str("consequat");
    s.push_str("Duis");
    s.push_str("aute");
    s.push_str("irure");
    s.push_str("dolor");
    s.push_str("in");
    s.push_str("reprehenderit");
    s.push_str("in");
    s.push_str("voluptate");
    s.push_str("velit");
    s.push_str("esse");
    s.push_str("cillum");
    s.push_str("dolore");
    s.push_str("eu");
    s.push_str("fugiat");
    s.push_str("nulla");
    s.push_str("pariatur");
    s.push_str("Excepteur");
    s.push_str("sint");
    s.push_str("occaecat");
    s.push_str("cupidatat");
    s.push_str("non");
    s.push_str("proident");
    s.push_str("sunt");
    s.push_str("in");
    s.push_str("culpa");
    s.push_str("qui");
    s.push_str("officia");
    s.push_str("deserunt");
    s.push_str("mollit");
    s.push_str("anim");
    s.push_str("id");
    s.push_str("est");
    s.push_str("laborum");
    s
}

// Benchmark all the above functions in a group for better comparison
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("lazy_str");
    group.bench_function("lazy_ipsum", |b| {
        b.iter(|| {
            lazy_ipsum();
        })
    });
    group.bench_function("normal_add_ipsum", |b| {
        b.iter(|| {
            normal_add_ipsum();
        })
    });
    group.bench_function("normal_push_ipsum", |b| {
        b.iter(|| {
            normal_push_ipsum();
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
