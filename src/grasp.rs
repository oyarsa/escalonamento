extern crate rand;

use std::u64;
use std::time::{Duration, Instant};
use self::rand::Rng;
use instancia::{Instancia, IdTarefa, Solucao, Sequencia};

#[allow(dead_code)]
pub fn solve(inst: &Instancia,
             alfa: f64, // 0.3 0.5 0.7
             timeout: Duration, // 30s
             num_vizinhos: u32, // 15 30 60
             max_iter: u64 /* INF */)
             -> (Solucao, u64, u64) {
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
        let vizinho = busca_local(&mut rng, inst, &atual, num_vizinhos);

        if vizinho.fo() < best.fo() {
            best = vizinho;
            it_alvo = it;
        }

        it += 1;
    }

    (best, it_alvo, it)
}

#[allow(dead_code)]
fn earliest_due_date<R: Rng + Sized>(rng: &mut R,
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
        let mut abertos: Vec<_> = (0..num_tarefas).filter(|t| !marcados[*t]).collect();
        abertos.sort_by_key(|t| inst.tarefa(*t).entrega());

        let num_candidatos = (abertos.len() as f64 * alfa).ceil() as usize;
        if num_candidatos == 0 {
            return None;
        }

        let proximo = abertos[rng.gen::<IdTarefa>() % num_candidatos];
        sequencia.push(proximo);

        marcados[proximo] = true;
        num_marcados += 1;
    }

    Some(sequencia)
}

#[allow(dead_code)]
fn construcao<R: Rng + Sized>(mut rng: &mut R, inst: &Instancia, alfa: f64) -> Solucao {
    loop {
        // if let Some(seq) = earliest_due_date(rng, inst, alfa) {
        //   return Solucao::new(inst, seq);
        // }
        if let Some(seq) = neh_semiguloso(rng, inst, alfa) {
            return seq;
        }
    }
}

fn neh_semiguloso<R: Rng + Sized>(rng: &mut R, inst: &Instancia, alfa: f64) -> Option<Solucao> {
    let mut sol = Solucao::new(&inst, vec![]);
    let n = inst.num_tarefas();
    let mut seq: Vec<_> = (0..n).collect();
    seq.sort_by_key(|t| -inst.tarefa(*t).entrega()); // EDD

    while !seq.is_empty() {
        let num_candidatos = (seq.len() as f64 * alfa).ceil() as usize;
        if num_candidatos == 0 {
            return None;
        }
        let tidx = rng.gen::<usize>() % num_candidatos;
        let t = seq.remove(tidx);

        let mut best: Option<Solucao> = None;
        for i in 0..sol.sequencia().len() + 1 {
            let mut v = sol.sequencia().clone();
            v.insert(i, t);
            let v = Solucao::new(&inst, v);
            if best.is_none() || v.fo() < best.as_ref().unwrap().fo() {
                best = Some(v);
            }
        }
        sol = best.unwrap();
    }

    Some(sol)
}

fn insercao<R: Rng + Sized>(rng: &mut R, mut seq: Sequencia) -> Sequencia {
    let tidx = rng.gen::<usize>() % seq.len();
    let nidx = rng.gen::<usize>() % seq.len();
    let t = seq.remove(tidx);
    seq.insert(nidx, t);
    seq
}

fn swap<R: Rng + Sized>(rng: &mut R, mut seq: Sequencia) -> Sequencia {
    let i = rng.gen::<usize>() % seq.len();
    let j = rng.gen::<usize>() % seq.len();
    seq.swap(i, j);
    seq
}

fn swap_adj<R: Rng + Sized>(rng: &mut R, mut seq: Sequencia) -> Sequencia {
    let i = rng.gen::<usize>() % (seq.len() - 1);
    let j = rng.gen::<usize>() % (seq.len() - 1);
    seq.swap(i, j);
    seq.swap(i + 1, j + 1);
    seq
}

fn swap_xyz<R: Rng + Sized>(rng: &mut R, mut seq: Sequencia) -> Sequencia {
    let y = rng.gen::<usize>() % (seq.len() - 2) + 1;
    let x = rng.gen::<usize>() % y;
    let z = rng.gen::<usize>() % (seq.len() - y - 1) + y;
    seq.swap(x, z);
    seq.swap(y, z);
    seq
}

fn vnd<R: Rng + Sized>(rng: &mut R,
                       inst: &Instancia,
                       solucao: &Solucao,
                       num_vizinhos: u32)
                       -> Solucao {
    let vizinhancas: Vec<fn(&mut R, Sequencia) -> Sequencia> = vec![insercao, swap, swap_adj,
                                                                    swap_xyz];
    let mut k = 0;
    let nv = vizinhancas.len();
    let mut sbest = solucao.clone();

    while k < nv {
        let sviz = best_improvement(rng, &sbest, inst, vizinhancas[k], num_vizinhos);
        if sviz.fo() < sbest.fo() {
            sbest = sviz;
            k = 0;
        } else {
            k += 1;
        }
    }

    sbest
}

fn best_improvement<R: Rng + Sized>(rng: &mut R,
                                    solucao: &Solucao,
                                    inst: &Instancia,
                                    operador: fn(&mut R, Sequencia) -> Sequencia,
                                    num_vizinhos: u32)
                                    -> Solucao {
    let mut best = solucao.clone();
    for _ in 0..num_vizinhos {
        let viz = Solucao::new(inst, operador(rng, solucao.sequencia().clone()));
        if viz.fo() < best.fo() {
            best = viz;
        }
    }
    best
}

#[allow(dead_code)]
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
fn busca_local<R: Rng + Sized>(rng: &mut R,
                               inst: &Instancia,
                               s: &Solucao,
                               num_vizinhos: u32)
                               -> Solucao {
    // for _ in 0..num_vizinhos {
    // s = busca_local_vizinho(inst, &s);
    // }
    // s

    vnd(rng, inst, s, num_vizinhos)
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
    pub fn solve(&self) -> (Solucao, u64, u64) {
        solve(self.inst,
              self.alfa,
              Duration::from_secs(self.timeout),
              self.num_vizinhos,
              self.max_iter)
    }
}
