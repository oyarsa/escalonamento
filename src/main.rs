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
    let (solucao, it) = Grasp::new(inst)
        .max_iter(INF as u64)
        .timeout(5)
        .alfa(0.3)
        .num_vizinhos(50)
        .solve();
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
        2...3 => Instancia::from_arquivo(&args[1]),
        _ => {
            println!("Opções inválidas");
            process::exit(1);
        }
    };

    match args.len() {
        3 => {
            match args[2].as_ref() {
                "-grasp" => teste_grasp(&inst),
                "-ag" => teste_ag(&inst),
                _ => {
                    println!("Algoritmo inválido");
                    process::exit(1);
                }
            }
        }
        _ => println!("Escolha um algoritmo (-grasp ou -ag)"),
    }

    // teste_ag(&inst);
    // teste_grasp(&inst);
    // teste();
}