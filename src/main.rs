use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Write};
use std::path::Path;

use rand::Rng;

use std::collections::HashMap;
use std::collections::HashSet;

fn read_file(filename: &str) -> Lines<BufReader<File>> {
    let path = Path::new(filename);
    let f = File::open(&path).expect("Unable to open file.");
    let buf = BufReader::new(f);
    return buf.lines();
}

fn clean_word(word: &str) -> String {
    return word
        .trim_start_matches(|c| char::is_ascii_punctuation(&c))
        .trim_end_matches(|c| char::is_ascii_punctuation(&c))
        .to_string();
}

fn _make_all() {
    let mut poe: VecDeque<String> = read_file("texts/poe.txt")
        .collect::<Result<_, _>>()
        .unwrap();
    let mut acd: VecDeque<String> = read_file("texts/sherlock.txt")
        .collect::<Result<_, _>>()
        .unwrap();
    let mut kiy: VecDeque<String> = read_file("texts/kinginyellow.txt")
        .collect::<Result<_, _>>()
        .unwrap();
    let mut hpl: VecDeque<String> = read_file("texts/hplovecraft.txt")
        .collect::<Result<_, _>>()
        .unwrap();

    println!("poe lines before: {}", poe.len());
    println!("acd lines before: {}", acd.len());
    println!("kiy lines before: {}", kiy.len());
    println!("hpl lines before: {}", hpl.len());

    let mut all: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    while poe.len() + acd.len() + kiy.len() + hpl.len() > 0 {
        let poe_len = u32::try_from(poe.len()).unwrap();
        let acd_len = u32::try_from(acd.len()).unwrap();
        let kiy_len = u32::try_from(kiy.len()).unwrap();
        let hpl_len = u32::try_from(hpl.len()).unwrap();
        let i = rng.gen_range(0..poe_len + acd_len + kiy_len + hpl_len);
        if i < poe_len {
            all.push(poe.pop_front().unwrap());
        } else if i < poe_len + acd_len {
            all.push(acd.pop_front().unwrap());
        } else if i < poe_len + acd_len + kiy_len {
            all.push(kiy.pop_front().unwrap());
        } else {
            all.push(hpl.pop_front().unwrap());
        }
    }

    println!("poe lines after: {}", poe.len());
    println!("acd lines after: {}", acd.len());
    println!("kiy lines after: {}", kiy.len());
    println!("hpl lines after: {}", hpl.len());
    println!("all lines: {}", all.len());

    let path = Path::new("texts/all.txt");
    let mut f = File::create(path).expect("Unable to create file.");
    for line in all {
        write!(f, "{}\n", line).expect("Failed to write line.");
    }
}

fn _make_all_l() {
    let mut poe: VecDeque<String> = read_file("texts/poe-l.txt")
        .collect::<Result<_, _>>()
        .unwrap();
    let mut acd: VecDeque<String> = read_file("texts/sherlock-l.txt")
        .collect::<Result<_, _>>()
        .unwrap();
    let mut kiy: VecDeque<String> = read_file("texts/kinginyellow-l.txt")
        .collect::<Result<_, _>>()
        .unwrap();
    let mut hpl: VecDeque<String> = read_file("texts/hplovecraft-l.txt")
        .collect::<Result<_, _>>()
        .unwrap();

    println!("poe lines before: {}", poe.len());
    println!("acd lines before: {}", acd.len());
    println!("kiy lines before: {}", kiy.len());
    println!("hpl lines before: {}", hpl.len());

    let mut all: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    while poe.len() + acd.len() + kiy.len() + hpl.len() > 0 {
        let poe_len = u32::try_from(poe.len()).unwrap();
        let acd_len = u32::try_from(acd.len()).unwrap();
        let kiy_len = u32::try_from(kiy.len()).unwrap();
        let hpl_len = u32::try_from(hpl.len()).unwrap();
        let i = rng.gen_range(0..poe_len + acd_len + kiy_len + hpl_len);
        if i < poe_len {
            all.push(poe.pop_front().unwrap());
        } else if i < poe_len + acd_len {
            all.push(acd.pop_front().unwrap());
        } else if i < poe_len + acd_len + kiy_len {
            all.push(kiy.pop_front().unwrap());
        } else {
            all.push(hpl.pop_front().unwrap());
        }
    }

    println!("poe lines after: {}", poe.len());
    println!("acd lines after: {}", acd.len());
    println!("kiy lines after: {}", kiy.len());
    println!("hpl lines after: {}", hpl.len());
    println!("all lines: {}", all.len());

    let path = Path::new("texts/all-l.txt");
    let mut f = File::create(path).expect("Unable to create file.");
    for line in all {
        write!(f, "{}\n", line).expect("Failed to write line.");
    }
}

fn _count_letters() {
    let mut all_letters: HashMap<char, u32> = HashMap::new();
    for line in read_file("texts/all.txt") {
        for c in line.unwrap().chars() {
            *all_letters.entry(c).or_insert(0) += 1;
        }
    }
    let mut count_all_letters: Vec<_> = all_letters.iter().collect();
    count_all_letters.sort_by(|a, b| b.1.cmp(a.1));
    println!("  All letters:");
    for entry in count_all_letters {
        println!("{}: {}", entry.1, entry.0);
    }

    let mut all_letters_l = HashMap::<char, u32>::new();
    for line in read_file("texts/all-l.txt") {
        for c in line.unwrap().chars() {
            *all_letters_l.entry(c).or_insert(0) += 1;
        }
    }
    println!("  Lowercase letters:");
    let mut count_all_letters_l: Vec<_> = all_letters_l.iter().collect();
    count_all_letters_l.sort_by(|a, b| b.1.cmp(a.1));
    for entry in count_all_letters_l {
        println!("{}: {}", entry.1, entry.0);
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // _make_all();
    // _make_all_l();

    // dictionaries

    println!("\n\tDictionaries:");

    let mut medium: HashSet<String> = HashSet::new();
    for line in read_file("dicts/medium-0.txt") {
        medium.insert(line.unwrap());
    }
    println!("medium: {}", medium.len());
    let mut medium_l: HashSet<String> = HashSet::new();
    for word in &medium {
        medium_l.insert(word.to_lowercase());
    }

    let mut default: HashSet<String> = HashSet::new();
    for line in read_file("dicts/default-1.txt") {
        default.insert(line.unwrap());
    }
    println!("default: {}", default.len());
    let mut default_l: HashSet<String> = HashSet::new();
    for word in &default {
        default_l.insert(word.to_lowercase());
    }

    let mut huge: HashSet<String> = HashSet::new();
    for line in read_file("dicts/huge-2.txt") {
        huge.insert(line.unwrap());
    }
    println!("huge: {}", huge.len());
    let mut huge_l: HashSet<String> = HashSet::new();
    for word in &huge {
        huge_l.insert(word.to_lowercase());
    }

    let mut insane: HashSet<String> = HashSet::new();
    for line in read_file("dicts/insane-3.txt") {
        insane.insert(line.unwrap());
    }
    println!("insane: {}", insane.len());
    let mut insane_l: HashSet<String> = HashSet::new();
    for word in &insane {
        insane_l.insert(word.to_lowercase());
    }
    // all words

    println!("\n\tCorpus: All");

    let mut corpus_words: HashSet<String> = HashSet::new();
    for line in read_file("texts/all.txt") {
        for word in line.unwrap().split_whitespace().map(clean_word) {
            corpus_words.insert(word);
        }
    }
    println!("corpus words: {}", corpus_words.len());

    let mut output_words: HashSet<String> = HashSet::new();
    for line in read_file("out/output-all.txt") {
        for word in line.unwrap().split_whitespace().map(clean_word) {
            output_words.insert(word);
        }
    }
    println!("output words: {}", output_words.len());

    let mut output_mostly_unique_words: HashSet<String> = HashSet::new();
    for word in &output_words {
        if !corpus_words.contains(word)
            && !medium.contains(word)
            && !default.contains(word)
            && !huge.contains(word)
        {
            output_mostly_unique_words.insert(word.to_string());
        }
    }
    let mut output_unique_words: HashSet<String> = HashSet::new();
    for word in &output_mostly_unique_words {
        if !insane.contains(word) {
            output_unique_words.insert(word.to_string());
        }
    }
    println!("mostly unique words: {}", output_mostly_unique_words.len());
    println!("fully unique words: {}", output_unique_words.len());
    for word in output_unique_words.iter().take(100) {
        println!("{}", word);
    }

    // lowercase words

    println!("\n\tCorpus: lowercase:");

    let mut corpus_words_l: HashSet<String> = HashSet::new();
    for line in read_file("texts/all-l.txt") {
        for word in line.unwrap().split_whitespace().map(clean_word) {
            corpus_words_l.insert(word);
        }
    }
    println!("corpus words lowercase: {}", corpus_words_l.len());

    let mut output_words_l: HashSet<String> = HashSet::new();
    for line in read_file("out/output-all-l.txt") {
        for word in line.unwrap().split_whitespace().map(clean_word) {
            output_words_l.insert(word);
        }
    }
    println!("output words lowercase: {}", output_words_l.len());

    let mut output_mostly_unique_words_l: HashSet<String> = HashSet::new();
    for word in &output_words_l {
        if !corpus_words_l.contains(word)
            && !medium_l.contains(word)
            && !default_l.contains(word)
            && !huge_l.contains(word)
        {
            output_mostly_unique_words_l.insert(word.to_string());
        }
    }
    let mut output_unique_words_l: HashSet<String> = HashSet::new();
    for word in &output_mostly_unique_words_l {
        if !insane_l.contains(word) {
            output_unique_words_l.insert(word.to_string());
        }
    }
    println!(
        "mostly unique words: {}",
        output_mostly_unique_words_l.len()
    );
    println!("fully unique words: {}", output_unique_words_l.len());
    for word in output_unique_words_l.iter().take(100) {
        println!("{}", word);
    }

    // _count_letters()

    // improvement: seen-words working hash-set, if seen skip immediately
    // next steps: strip 's off words
    // build database schema to manually evaluate words, "no, yes, good maybe, bad maybe"?
}
