use std::env;
use std::fs::File;
use std::io::Read;
use std_test::Bencher;

use ::std::collections::HashMap;
use radix_trie::Trie;
use sequence_trie::SequenceTrie;

#[cfg(test)]
fn get_text() -> Vec<String> {
    let mut contents = String::new();
    let filename = env::var("TEST_FILE").ok().expect("$TEST_FILE undefined.");
    File::open(&filename).unwrap().read_to_string(&mut contents).unwrap();
    contents.split(|c: char| c.is_whitespace()).map(|s| s.to_string()).collect()
}

#[test]
fn radix_trie_integrity() {
    let words = get_text();

    let mut trie = Trie::<&str, usize>::new();

    for w in &words {
        trie.insert(&w[..], w.len());

        if !trie.check_integrity() {
            panic!("Trie invariant violated upon insertion of \"{}\"'", w);
        }
    }
}

#[bench]
fn hashmap_insert(b: &mut Bencher) {
    let words = get_text();

    b.iter(|| {
        let mut map = HashMap::<&str, usize>::new();

        for w in &words {
            map.insert(&w[..], w.len());
        }
    });
}

#[bench]
fn radix_trie_insert(b: &mut Bencher) {
    let words = get_text();

    b.iter(|| {
        let mut trie = Trie::<&str, usize>::new();

        for w in &words {
            trie.insert(&w[..], w.len());
        }
    });
}

#[bench]
fn sequence_trie_insert(b: &mut Bencher) {
    let words: Vec<Vec<char>> = get_text().iter().map(|s| s.chars().collect()).collect();

    b.iter(|| {
        let mut trie = SequenceTrie::new();

        for w in &words {
            trie.insert(&w[..], w.len());
        }
    });
}
