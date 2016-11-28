mod grasp;
mod instancia;
mod ag;

use std::env;
use std::io;
use std::process;
use std::time::Instant;
use instancia::{Instancia, INF};
use grasp::Grasp;
use ag::{Ag, Cruzamento, Mutacao};

const NUM_EXEC: u32 = 10;

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

fn teste(inst: Instancia) {
    teste_ag(&inst);
    teste_grasp(&inst);
}

fn experimento_grasp(inst: Instancia, config: &[&str]) {
    let id = config[0];
    let alfa: f64 = config[1].parse().expect("Erro ao ler o Alfa do GRASP");
    let num_vizinhos: u32 = config[2].parse().expect("Erro ao ler o número de vizinhos do GRASP");

    let mut grasp = Grasp::new(&inst);
    grasp.alfa(alfa).num_vizinhos(num_vizinhos).max_iter(INF as u64).timeout(30);

    println!("ID, iExec, FO, IterAlvo, Tempo");
    for i in 0..NUM_EXEC {
        let t = Instant::now();
        let (solucao, iter) = grasp.solve();
        let tempo = t.elapsed();

        println!("{}, {}, {}, {}, {}.{}",
                 id,
                 i,
                 solucao.fo(),
                 iter,
                 tempo.as_secs(),
                 tempo.subsec_nanos());
    }
}

fn experimento_ag(inst: Instancia, config: &[&str]) {
    let id = config[0];
    let pop_tam: usize = config[1].parse().expect("Erro ao ler tamanho da população do AG");
    let xo_chance: f64 = config[2].parse().expect("Erro ao ler chance de cruzamento do AG");
    let cruz: u32 = config[3].parse().expect("Erro ao ler operador de cruzamento do AG");
    let mutacao: u32 = config[4].parse().expect("Erro ao ler operador de mutação do AG");
    let mut_chance: f64 = config[5].parse().expect("Erro ao ler chance de mutação do AG");

    let cruz = match cruz {
        0 => Cruzamento::PMX,
        1 => Cruzamento::OX,
        _ => {
            println!("Cruzamento inválido");
            process::exit(1);
        }
    };

    let mutacao = match mutacao {
        0 => Mutacao::Swap,
        1 => Mutacao::TwoOpt,
        _ => {
            println!("Mutação inválida");
            process::exit(1);
        }
    };

    let mut ag = Ag::new(&inst);
    ag.pop_tam(pop_tam).xo_chance(xo_chance).cruz(cruz).mutacao(mutacao).mut_chance(mut_chance);

    println!("ID, iExec, FO, IterAlvo, Tempo");
    for i in 0..NUM_EXEC {
        let t = Instant::now();
        let (solucao, iter) = ag.solve();
        let tempo = t.elapsed();

        println!("{}, {}, {}, {}, {}.{}",
                 id,
                 i,
                 solucao.fo(),
                 iter,
                 tempo.as_secs(),
                 tempo.subsec_nanos());
    }
}

fn experimento(inst: Instancia) {
    let mut config = String::new();

    io::stdin().read_line(&mut config).expect("Erro ao ler configuração");
    let config: Vec<_> = config.split_whitespace().collect();

    match config[0].as_ref() {
        "grasp" => experimento_grasp(inst, &config[1..]),
        "ag" => experimento_ag(inst, &config[1..]),
        _ => {
            println!("Algoritmo inválido");
            process::exit(1);
        }
    }
}

fn print_usage() {
    let usage = "
    Experimento: ./escalonamento <entrada> -e

    A configuração será lida da entrada padrão

    Formato da configuração do GRASP:
        grasp ID Alfa NumVizinhos

    Formato da configuração do AG:
        ag ID PopTam XoChance Cruz Mut MutChance
    Onde
        Cruz = 0 (PMX) ou 1 (OX)
        Mut = 0 (Swap) ou 1 (2-opt)
    ";

    println!("{}", usage);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let inst: Instancia = match args.len() {
        1 => Instancia::toy(),
        2...3 => {
            if args[1] == "-h" {
                print_usage();
                process::exit(0);
            } else {
                Instancia::from_arquivo(&args[1])
            }
        }
        _ => {
            println!("Opções inválidas");
            process::exit(1);
        }
    };

    match args.len() {
        2 => teste(inst),
        3 => {
            match args[2].as_ref() {
                "-grasp" => teste_grasp(&inst),
                "-ag" => teste_ag(&inst),
                "-e" => experimento(inst),
                _ => {
                    println!("Algoritmo inválido");
                    process::exit(1);
                }
            }
        }
        _ => println!("Escolha um algoritmo (-grasp ou -ag)"),
    }
}