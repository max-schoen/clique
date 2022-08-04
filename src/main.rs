mod graph;
mod clique;

fn main() {
    let words = graph::build_graph_from_file("res/words_alpha.txt");
    let cliques = clique::find_cliques(words);
    println!("Found {} cliques!", cliques.len());
}
