// The following function contains the path to a list of english words, one per
// line.  The location of this file may vary per system, so you may need to
// adjust this path accordingly.

pub fn solve(letters: &str) -> Vec<String> {
    let scowl = std::env!("SCOWL");
    let words_path = format!("{}/share/dict/words.txt", scowl);
    let mut words: Vec<String> = std::fs::read(words_path)
        .as_ref()
        .and_then(|words| Ok(String::from_utf8_lossy(words)))
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    words.sort_by_key(|w| w.len());
    words.reverse();
    let letter_sig: Sig = Sig::new(letters.trim());
    words
        .into_iter()
        .filter(|w| letter_sig.contains(&Sig::new(w)))
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

#[derive(Debug)]
struct Sig(Vec<char>);

// The signature of a word, which is just all of the letters in
// alphabetical order. signatures have two useful properties.  Firstly, it's
// easy to compare two signatures to see if one contains the other, and
// secondly, for two collections of letters A and B, A contains all the letters
// in B if sig(A) contains sig(B).

impl Sig {
    fn new(s: &str) -> Sig {
        let mut cs: Vec<char> = s.chars().collect();
        cs.sort();
        Sig(cs)
    }

    fn contains(&self, small: &Sig) -> bool {
        let mut i = 0;
        let mut j = 0;
        loop {
            if j == small.0.len() {
                return true;
            } else if i == self.0.len() {
                return false;
            } else if self.0[i] == small.0[j] {
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
        }
    }
}
#[test]
fn test_sig_contains() {
    assert!(Sig::new("aaabbc").contains(&Sig::new("abc")));
    assert!(Sig::new("aaabbc").contains(&Sig::new("aaabc")));
    assert!(false == Sig::new("aaabbc").contains(&Sig::new("aaaabc")));
    assert!(Sig::new("aaabbc").contains(&Sig::new("aaa")));
    assert!(Sig::new("aaabbc").contains(&Sig::new("ac")));
    assert!(Sig::new("aaabbc").contains(&Sig::new("ca")));
    assert!(Sig::new("aaabbc").contains(&Sig::new("bac")));
}
