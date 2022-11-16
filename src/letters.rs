// The following function contains the path to a list of english words, one per
// line.  The location of this file may vary per system, so you may need to
// adjust this path accordingly.

pub fn solve(letters: &str) -> Vec<String> {
    let mut words: Vec<String> = std::fs::read_to_string("/usr/share/dict/words")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    words.sort_by_key(|w| w.len());
    words.reverse();
    let letter_sig: Vec<char> = sig(letters.trim());
    words
        .into_iter()
        .filter(|w| sig_contains(&letter_sig, &sig(w)))
        .take(5)
        .collect()
}

pub fn do_letters_puzzle() {
    println!("Enter a collection of letters with no spaces in between.");
    let mut letters = String::new();
    std::io::stdin().read_line(&mut letters).unwrap();
    for word in solve(&letters).iter() {
        println!("{} {}", word.len(), word);
    }
}

// Compute the signature of a word, which is just all of the letters in
// alphabetical order. signatures have two useful properties.  Firstly, it's
// easy to compare two signatures to see if one contains the other, and
// secondly, for two collections of letters A and B, A contains all the letters
// in B if sig(A) contains sig(B).

fn sig(s: &str) -> Vec<char> {
    let mut cs: Vec<char> = s.chars().collect();
    cs.sort();
    cs
}

fn sig_contains(big: &Vec<char>, small: &Vec<char>) -> bool {
    let mut i = 0;
    let mut j = 0;
    loop {
        if j == small.len() {
            return true;
        } else if i == big.len() {
            return false;
        } else if big[i] == small[j] {
            i += 1;
            j += 1;
        } else {
            i += 1;
        }
    }
}
#[test]
fn test_sig_contains() {
    assert!(sig_contains(&sig("aaabbc"), &sig("abc")));
    assert!(sig_contains(&sig("aaabbc"), &sig("aaabc")));
    assert!(false == sig_contains(&sig("aaabbc"), &sig("aaaabc")));
    assert!(sig_contains(&sig("aaabbc"), &sig("aaa")));
    assert!(sig_contains(&sig("aaabbc"), &sig("ac")));
    assert!(sig_contains(&sig("aaabbc"), &sig("ca")));
    assert!(sig_contains(&sig("aaabbc"), &sig("bac")));
}
