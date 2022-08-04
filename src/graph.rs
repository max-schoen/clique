use std::collections::{HashSet, HashMap};
use std::fmt::Write;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use std::fs;
use rayon::prelude::*;
use console::{style, Emoji};

#[derive(Eq, Debug)]
pub struct Word {
    pub word: String,
    pub char_set: HashSet<char>,
    pub neighbors: HashSet<String>,
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        return self.word.eq(&other.word);
    }
}

impl Hash for Word {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.word.hash(state)
    }
}

impl Word {
    fn new(word: String, neighbors: Option<HashSet<String>>) -> Word {
        let char_set: HashSet<char> = word.chars().fold(HashSet::<char>::new(), |mut acc: HashSet<char>, w| { acc.insert(w); acc });
        Word {
            word,
            char_set,
            neighbors: neighbors.unwrap_or_else(|| HashSet::new()),
        }
    }

    fn get_char_set(word: &str) -> HashSet<char> {
        word.chars().fold(HashSet::<char>::new(), |mut acc: HashSet<char>, w| { acc.insert(w); acc })
    }

    fn from_str(word: &str) -> Word {
        Word::new(word.to_string(), None)
    }

    pub fn to_string(&self) -> String {
        return self.word.clone();
    }

    pub fn neighbors_to_string(&self) -> String {
        return format!("{}", itertools::join(&self.neighbors, ", "));
    }
}

fn compute_neighbors(words: HashSet<Word>) -> HashMap<String,Word> {
    let words = Arc::new(words);
    let results = Arc::new(Mutex::new(HashMap::<String,Word>::new()));
    let pb = Arc::new(Mutex::new(ProgressBar::new(words.len() as u64)));
    println!("{} {}Building graph...", style("[1/2]").bold().dim(), Emoji("🕸 ", ""));
    pb.lock().unwrap().set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .tick_chars("▉▊▋▌▍▎▏▎▍▌▋▊▉")
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
    words.clone().par_iter().for_each(|word| {
        let mut neighbors = HashSet::<String>::new();
        for neighbor in words.clone().iter() {
            if word.char_set.intersection(&neighbor.char_set).count() == 0 {
                neighbors.insert(neighbor.word.clone());
            }
        }
        results.lock().unwrap().insert(word.word.clone(), Word::new(word.word.clone(), Some(neighbors)));
        pb.lock().unwrap().inc(1);
    });
    pb.lock().unwrap().finish();
    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

pub fn build_graph_from_file(path: &str) -> HashMap<String,Word> {
    let words = fs::read_to_string(path).unwrap();
    let words = words.split_whitespace().filter(|w| w.chars().count() == 5 && Word::get_char_set(w).len() == 5);
    let words = words.fold(HashSet::<Word>::new(), |mut acc, w| { acc.insert(Word::from_str(w)); acc });
    compute_neighbors(words)
}

