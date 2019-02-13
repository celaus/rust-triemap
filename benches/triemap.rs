#[macro_use]
extern crate criterion;
use criterion::Criterion;

use criterion::Bencher;
use rand::{thread_rng, Rng};
use triemap::TrieMap;

const ITEMS: usize = 10_000;

fn bench_triemap_find_success(crit: &mut Criterion) {
    crit.bench_function("Triemap", |b| {
        let mut trie = TrieMap::<u8, usize>::new_empty();

        let mut values = vec![];
        for i in 0..ITEMS {
            let s = format!("{:08}", i).into_bytes();
            trie.insert(&s, i);
            values.push((s, i));
        }
        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.choose(&values).unwrap();
            trie.get(&r.0).expect("NOT FOUND")
        });
    });
}

fn bench_btreemap_find_success(crit: &mut Criterion) {
    crit.bench_function("BTreeMap", |b| {
        let mut tree = std::collections::BTreeMap::new();

        let mut values = vec![];
        for i in 0..ITEMS {
            let s = format!("{:08}", i).into_bytes();
            tree.insert(s.clone(), i);
            values.push((s, i));
        }
        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.choose(&values).unwrap();
            tree.get(&r.0).expect("NOT FOUND")
        });
    });
}

criterion_group!(
    benches,
    bench_triemap_find_success,
    bench_btreemap_find_success
);
criterion_main!(benches);
