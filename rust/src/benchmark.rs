use std::env;
use std::fs::File;
use std::io::Read;
use std_test::Bencher;

use radix_trie::Trie;

#[cfg(test)]
fn get_text() -> Vec<String> {
    let mut contents = String::new();
    let filename = env::var("TEST_FILE").ok().expect("$TEST_FILE undefined.");
    File::open(&filename).unwrap().read_to_string(&mut contents).unwrap();
    contents.split(|c: char| c.is_whitespace()).map(|s| s.to_string()).collect()
}

#[cfg(test)]
fn make_trie(words: &[String]) -> Trie<&str, usize> {
    let mut trie = Trie::new();

    for w in words {
        trie.insert(&w[..], w.len());
    }

    trie
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
fn trie_insert(b: &mut Bencher) {
    let words = get_text();

    b.iter(|| {
        make_trie(&words);
    });
}

#[bench]
fn trie_get(b: &mut Bencher) {
    let words = get_text();
    let trie = make_trie(&words);

    b.iter(|| {
        for w in &words {
            trie.get(&&w[..]);
        }
    });
}

#[bench]
fn trie_insert_remove(b: &mut Bencher) {
    let words = get_text();

    b.iter(|| {
        let mut trie = make_trie(&words);

        for w in &words {
            trie.remove(&&w[..]);
        }
    });
}
