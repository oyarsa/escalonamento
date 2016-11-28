mod grasp;
mod instancia;
mod ag;

use std::env;
use std::process;
use std::time::Instant;
use instancia::{Instancia, INF};
use grasp::Grasp;
use ag::Ag;

#[allow(dead_code)]
fn teste_grasp(inst: &Instancia) {
    println!("Grasp");
    let t = Instant::now();
    let (solucao, it) = Grasp::new(inst).max_iter(40).timeout(INF as u64).solve();
    let tempo = t.elapsed();

    println!("Sequencia: {:?}", solucao.sequencia());
    println!("Iteração alvo: {}", it);
    println!("Fo: {}", solucao.fo());
    println!("Tempo: {}.{}", tempo.as_secs(), tempo.subsec_nanos());
    println!("-------------------\n");
}

#[allow(dead_code)]
fn teste_ag(inst: &Instancia) {
    println!("AG");
    let t = Instant::now();
    let (solucao, it) = Ag::new(inst)
        .max_iter(INF as u64)
        .timeout(5)
        .mut_chance(0.3)
        .pop_tam(1000)
        .xo_chance(1.0)
        .solve();
    let tempo = t.elapsed();

    println!("Sequencia: {:?}", solucao.sequencia());
    println!("Iteração alvo: {}", it);
    println!("Fo: {}", solucao.fo());
    println!("Tempo: {}.{}", tempo.as_secs(), tempo.subsec_nanos());
    println!("-------------------\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let inst: Instancia = match args.len() {
        1 => Instancia::toy(),
        2 => Instancia::from_arquivo(&args[1]),
        _ => {
            println!("Opções inválidas");
            process::exit(1);
        }
    };

    teste_ag(&inst);
    // teste_grasp(&inst);
    // teste();
}