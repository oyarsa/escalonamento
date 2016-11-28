extern crate rand;

use std::u64;
use std::time::{Duration, Instant};
use self::rand::Rng;
use instancia::{Instancia, IdTarefa, Solucao, Sequencia};

#[allow(dead_code)]
pub fn solve(inst: &Instancia,
             alfa: f64, // 0.3 0.5 0.7
             timeout: Duration, // 30s
             num_vizinhos: u32, // 5 10 15
             max_iter: u64)
             -> (Solucao, u64) {
    let mut rng = rand::weak_rng();
    let t = Instant::now();

    let mut it = 0;
    let mut it_alvo = 0;
    let mut best = Solucao::vazia();

    while it - it_alvo < max_iter && t.elapsed() < timeout {
        if it % max_iter == 0 {
            println!("i: {}", it);
        }

        let atual = construcao(&mut rng, inst, alfa);
        let vizinho = busca_local(inst, atual, num_vizinhos);

        if vizinho.fo() < best.fo() {
            best = vizinho;
            it_alvo = it;
        }

        it += 1;
    }

    (best, it_alvo)
}

#[allow(dead_code)]
fn vizinho_mais_proximo<R: Rng + Sized>(mut rng: &mut R,
                                        inst: &Instancia,
                                        alfa: f64)
                                        -> Option<Sequencia> {
    let num_tarefas = inst.num_tarefas();
    let mut sequencia = Vec::with_capacity(num_tarefas);
    let mut marcados = vec![false; num_tarefas];
    let mut num_marcados = 0;

    let inicial = rng.gen::<IdTarefa>() % num_tarefas;
    sequencia.push(inicial);
    marcados[inicial] = true;
    num_marcados += 1;

    while num_marcados < num_tarefas {
        let atual = sequencia[sequencia.len() - 1];
        let abertos: Vec<(IdTarefa, usize)> = vec![];
        let num_candidatos = (abertos.len() as f64 * alfa).ceil() as usize;
        if num_candidatos == 0 {
            return None;
        }

        let (proximo, _) = abertos[rng.gen::<IdTarefa>() % num_candidatos];
        sequencia.push(proximo);
        marcados[proximo] = true;
        num_marcados += 1;
    }

    Some(sequencia)
}

#[allow(dead_code)]
fn construcao<R: Rng + Sized>(mut rng: &mut R, inst: &Instancia, alfa: f64) -> Solucao {
    loop {
        if let Some(seq) = vizinho_mais_proximo(&mut rng, inst, alfa) {
            return Solucao::new(inst, seq);
        }
    }
}
fn busca_local_vizinho(inst: &Instancia, solucao: &Solucao) -> Solucao {
    let mut atual = solucao.clone();
    while let Some(nova) = two_opt_loop(inst, &atual) {
        atual = nova;
    }
    atual
}

fn two_opt_swap(mut sequencia: Sequencia, i: usize, k: usize) -> Sequencia {
    sequencia[i..k].reverse();
    sequencia
}

#[allow(dead_code)]
fn two_opt_loop(inst: &Instancia, solucao: &Solucao) -> Option<Solucao> {
    let num_tarefas = inst.num_tarefas();
    let mut best = solucao.clone();

    for i in 0..num_tarefas - 1 {
        for k in i + 1..num_tarefas {
            let nova = two_opt_swap(solucao.sequencia().clone(), i, k);
            let nova = Solucao::new(inst, nova);
            if nova.fo() < best.fo() {
                best = nova;
            }
        }
    }

    if best.fo() < solucao.fo() {
        Some(best)
    } else {
        None
    }
}

#[allow(dead_code)]
fn busca_local(inst: &Instancia, s: Solucao, num_vizinhos: u32) -> Solucao {
    (0..num_vizinhos)
        .map(|_| busca_local_vizinho(inst, &s))
        .min_by_key(Solucao::fo)
        .unwrap_or(s)
}

pub struct Grasp<'a> {
    inst: &'a Instancia,
    alfa: f64,
    timeout: u64,
    num_vizinhos: u32,
    max_iter: u64,
}

impl<'a> Grasp<'a> {
    #[allow(dead_code)]
    pub fn new(inst: &Instancia) -> Grasp {
        Grasp {
            inst: inst,
            alfa: 0.35,
            timeout: u64::MAX,
            num_vizinhos: 10,
            max_iter: 40,
        }
    }

    #[allow(dead_code)]
    pub fn alfa(&mut self, alfa: f64) -> &mut Grasp<'a> {
        self.alfa = alfa;
        self
    }

    #[allow(dead_code)]
    pub fn timeout(&mut self, timeout: u64) -> &mut Grasp<'a> {
        self.timeout = timeout;
        self
    }

    #[allow(dead_code)]
    pub fn num_vizinhos(&mut self, num_vizinhos: u32) -> &mut Grasp<'a> {
        self.num_vizinhos = num_vizinhos;
        self
    }

    #[allow(dead_code)]
    pub fn max_iter(&mut self, max_iter: u64) -> &mut Grasp<'a> {
        self.max_iter = max_iter;
        self
    }

    #[allow(dead_code)]
    pub fn solve(&self) -> (Solucao, u64) {
        solve(self.inst,
              self.alfa,
              Duration::from_secs(self.timeout),
              self.num_vizinhos,
              self.max_iter)
    }
}
