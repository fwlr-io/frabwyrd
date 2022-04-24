use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Lines, Write};
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

fn lowercase_strip_word(word: &str) -> String {
    return word
        .trim_start_matches(|c| char::is_ascii_punctuation(&c))
        .trim_end_matches(|c| char::is_ascii_punctuation(&c))
        .trim_end_matches("'s")
        .to_lowercase()
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
    let mut medium: HashSet<String> = HashSet::new();
    for line in read_file("dicts/medium-0.txt") {
        let word = line.unwrap();
        medium.insert(word.to_string());
        medium.insert(word.to_lowercase());
        medium.insert(clean_word(&word));
        medium.insert(lowercase_strip_word(&word));
    }
    let mut default: HashSet<String> = HashSet::new();
    for line in read_file("dicts/default-1.txt") {
        let word = line.unwrap();
        default.insert(word.to_string());
        default.insert(word.to_lowercase());
        default.insert(clean_word(&word));
        default.insert(lowercase_strip_word(&word));
    }
    let mut huge: HashSet<String> = HashSet::new();
    for line in read_file("dicts/huge-2.txt") {
        let word = line.unwrap();
        huge.insert(word.to_string());
        huge.insert(word.to_lowercase());
        huge.insert(clean_word(&word));
        huge.insert(lowercase_strip_word(&word));
    }
    let mut insane: HashSet<String> = HashSet::new();
    for line in read_file("dicts/insane-3.txt") {
        let word = line.unwrap();
        insane.insert(word.to_string());
        insane.insert(word.to_lowercase());
        insane.insert(clean_word(&word));
        insane.insert(lowercase_strip_word(&word));
    }
    println!(
        "Dictionaries:\t\tmedium: {}\t\tdefault: {}\t\thuge: {}\t\tinsane: {}",
        medium.len(),
        default.len(),
        huge.len(),
        insane.len()
    );

    // lowercase words

    let mut corpus_words_l: HashSet<String> = HashSet::new();
    for line in read_file("texts/all-l.txt") {
        for word in line.unwrap().split_whitespace() {
            corpus_words_l.insert(word.to_string());
            corpus_words_l.insert(word.to_lowercase());
            corpus_words_l.insert(clean_word(word));
            corpus_words_l.insert(lowercase_strip_word(word));
        }
    }
    let mut output_words_l: HashSet<String> = HashSet::new();
    for line in read_file("out/output-all-l.txt") {
        for word in line.unwrap().split_whitespace() {
            output_words_l.insert(lowercase_strip_word(word));
        }
    }
    let mut output_unique_words_l: HashSet<String> = HashSet::new();
    for raw_word in &output_words_l {
        // improvement: seen-words working hash-set, if seen skip immediately
        let word = lowercase_strip_word(raw_word);
        if !corpus_words_l.contains(&word)
            && !medium.contains(&word)
            && !default.contains(&word)
            && !huge.contains(&word)
            && !insane.contains(&word)
            && !word.contains(|c| char::is_numeric(c))
            && !word.contains(|c| char::is_ascii_punctuation(&c))
        {
            output_unique_words_l.insert(word.to_string());
        }
    }
    println!(
        "Lowercase corpus:\tcorpus words: {}\t\toutput words: {}\t\tunique words: {}",
        corpus_words_l.len(),
        output_words_l.len(),
        output_unique_words_l.len()
    );

    println!("\n    SOME RANDOM WORDS:");
    for (i, word) in output_words_l.iter().take(50).enumerate() {
        print!("{}\t", word);
        if (i + 1) % 10 == 0 {
            println!();
        }
    }

    println!("\n    SOME SHORT WORDS:");
    for (i, word) in output_words_l
        .iter()
        .filter(|c| c.len() < 6)
        .take(75)
        .enumerate()
    {
        print!("{}\t", word);
        if (i + 1) % 15 == 0 {
            println!();
        }
    }

    println!("\n    SOME LONG WORDS:");
    for (i, word) in output_words_l
        .iter()
        .filter(|c| c.len() > 15)
        .take(25)
        .enumerate()
    {
        print!("{}\t", word);
        if (i + 1) % 5 == 0 {
            println!();
        }
    }
    stdout().flush().expect("Cannot flush stdout");

    // _count_letters()

    // build database? schema to manually evaluate words, "no, yes, good maybe, bad maybe"?
}
