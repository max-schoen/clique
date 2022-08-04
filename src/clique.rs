use std::{collections::{HashSet, HashMap}, sync::{Arc, Mutex}, fmt::Write};

use console::{style, Emoji};
use indicatif::{ProgressStyle, ProgressBar, ProgressState};
use rayon::prelude::*;

use crate::graph::Word;

pub fn find_cliques(words: HashMap<String,Word>) -> Vec<Box<[String]>> {
    let words = Arc::new(words);
    let results: Arc<Mutex<Vec<Box<[String]>>>> = Arc::new(Mutex::new(Vec::new()));
    let pb = Arc::new(Mutex::new(ProgressBar::new(words.len() as u64)));
    println!("{} {}Finding cliques...", style("[2/2]").bold().dim(), Emoji("👯‍♀️ ", ""));
    pb.lock().unwrap().set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .tick_chars("▉▊▋▌▍▎▏▎▍▌▋▊▉")
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}min", state.eta().as_secs_f64() / 60.0).unwrap())
        .progress_chars("#>-"));
    words.clone().par_iter().for_each(|(word_i, word)| {
        let neighbors_i = &word.neighbors;
        for word_j in neighbors_i {
            if word_j < word_i { continue; }
            let words = words.clone();
            let neighbors_ij: HashSet<String> = neighbors_i.intersection(&words.get(word_j).unwrap().neighbors).cloned().collect();
            if neighbors_ij.len() < 3 { continue; }
            for word_k in &neighbors_ij {
                if word_k < word_j { continue; }
                let words = words.clone();
                let neighbors_ijk: HashSet<String> =  neighbors_ij.intersection(&words.get(word_k).unwrap().neighbors).cloned().collect();
                if neighbors_ijk.len() < 2 { continue; }
                for word_l in &neighbors_ijk {
                    if word_l < word_k { continue; }
                    let words = words.clone();
                    let neighbors_ijkl: HashSet<String> = neighbors_ijk.intersection(&words.get(word_l).unwrap().neighbors).cloned().collect();
                    for word_r in &neighbors_ijkl {
                        if word_r < word_l { continue; }
                    //    println!("{:?}", [word_i, word_j, word_k, word_l, word_r]);
                        results.lock().unwrap().push(Box::new([word_i.clone(), word_j.clone(), word_k.clone(), word_l.clone(), word_r.clone()]));
                    }
                }
            }
        }
        pb.lock().unwrap().inc(1);
    });
    pb.lock().unwrap().finish();
    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}