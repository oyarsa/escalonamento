extern crate rand;

use std::u64;
use std::time::{Duration, Instant};
use std::cmp::{min, max};
use self::rand::Rng;
use instancia::{Instancia, IdTarefa, Solucao, Sequencia};

type Populacao = Vec<Solucao>;

#[derive(Clone, Copy)]
pub enum Cruzamento {
    OX,
    PMX,
}

#[derive(Clone, Copy)]
pub enum Mutacao {
    Swap,
    TwoOpt,
}

#[allow(dead_code)]
pub fn solve(inst: &Instancia,
             timeout: Duration, // 30s
             max_iter: u64, // INF
             pop_tam: usize, // 250, 500
             xo_chance: f64, // 0.95, 0.99
             cruz: Cruzamento, // metodo de cruzamento: OX, PMX
             mutacao: Mutacao, // método de mutação: Swap, 2-opt
             mut_chance: f64 /* 0.05 0.10 0.20 */)
             -> (Solucao, u64, u64) {
    let mut rng = rand::weak_rng();
    let mut pop = populacao_inicial(&mut rng, inst, pop_tam);
    let mut best_fo = pop[0].fo();
    let mut it = 0;
    let mut it_melhor = 0;
    let xo_num = ((xo_chance * pop_tam as f64) / 2.0).floor() as usize;
    let t = Instant::now();

    while it - it_melhor < max_iter && t.elapsed() < timeout {
        let filhos;
        {
            let pais = selecao(&mut rng, &pop, xo_num);
            filhos = recombinacao(&mut rng, inst, pais, mut_chance, cruz, mutacao);
        }
        pop = proxima_geracao(pop, filhos, pop_tam);

        if pop[0].fo() < best_fo {
            it_melhor = it;
            best_fo = pop[0].fo();
        }
        it += 1;
    }

    (pop.swap_remove(0), it_melhor, it)
}

#[allow(dead_code)]
fn gen_roleta(pop: &Populacao) -> Vec<f32> {
    let total = pop.iter().map(|s| 1.0 / s.fo() as f32).sum::<f32>();
    pop.iter()
        .scan(0.0, |state, prob| {
            *state += prob.fo() as f32 / total;
            Some(*state)
        })
        .collect()
}

#[allow(dead_code)]
fn get_index_from_roleta<R: Rng + Sized>(mut rng: &mut R, roleta: &[f32]) -> usize {
    let x = rng.next_f32();
    for (i, &prob) in roleta.iter().enumerate() {
        if x <= prob {
            return i;
        }
    }
    0
}

#[allow(dead_code)]
fn seleciona_pais<'a, R: Rng + Sized>(mut rng: &mut R,
                                      pop: &'a Populacao,
                                      roleta: &[f32])
                                      -> (&'a Sequencia, &'a Sequencia) {
    let pai1 = pop[get_index_from_roleta(rng, roleta)].sequencia();
    let pai2 = pop[get_index_from_roleta(rng, roleta)].sequencia();
    (pai1, pai2)
}

#[allow(dead_code)]
fn selecao<'a, R: Rng + Sized>(rng: &mut R,
                               pop: &'a Populacao,
                               xo_num: usize)
                               -> Vec<(&'a Sequencia, &'a Sequencia)> {
    let roleta = gen_roleta(pop);
    (0..xo_num)
        .map(|_| seleciona_pais(rng, pop, &roleta))
        .collect()
}

#[allow(dead_code)]
fn proxima_geracao(atual: Populacao, mut filhos: Populacao, pop_tam: usize) -> Populacao {
    let mut proxima = atual;
    let begin = pop_tam - filhos.len();

    for i in begin..pop_tam {
        proxima[i] = filhos.pop().expect("Erro ao gerar próxima geração");
    }

    proxima.sort_by_key(Solucao::fo);
    proxima
}

#[allow(dead_code)]
fn populacao_inicial<R: Rng + Sized>(rng: &mut R, inst: &Instancia, pop_tam: usize) -> Populacao {
    let mut pop: Vec<_> = (0..pop_tam).map(|_| individuo_aleatorio(rng, inst)).collect();
    pop.sort_by_key(Solucao::fo);
    pop
}

#[allow(dead_code)]
fn individuo_aleatorio<R: Rng + Sized>(mut rng: &mut R, inst: &Instancia) -> Solucao {
    Solucao::new(inst, sequencia_aleatoria(rng, inst))
}

#[allow(dead_code)]
fn sequencia_aleatoria<R: Rng + Sized>(mut rng: &mut R, inst: &Instancia) -> Sequencia {
    let num_tarefas = inst.num_tarefas();
    let mut seq: Vec<_> = (0..num_tarefas).collect();
    rng.shuffle(seq.as_mut_slice());
    seq
}

#[allow(dead_code)]
fn two_opt_aleatorio<R: Rng + Sized>(mut rng: &mut R, mut sequencia: Sequencia) -> Sequencia {
    let (i, k) = gen_points(rng, sequencia.len());
    sequencia[i..k].reverse();
    sequencia
}

#[allow(dead_code)]
fn gen_points<R: Rng + Sized>(mut rng: &mut R, num_tarefas: usize) -> (IdTarefa, IdTarefa) {
    let i = rng.gen::<IdTarefa>() % num_tarefas;
    let j = rng.gen::<IdTarefa>() % num_tarefas;

    (min(i, j), max(i, j))
}

fn pmx_crossover<R: Rng + Sized>(mut rng: &mut R, pai1: &Sequencia, pai2: &Sequencia) -> Sequencia {
    let num_tarefas = pai1.len();

    let mut genes = pai1.clone();
    let mut map = vec![0; num_tarefas + 1];
    let (xbegin, xend) = gen_points(rng, num_tarefas);

    for (i, &vert) in genes.iter().enumerate() {
        map[vert] = i;
    }

    for i in xbegin..xend {
        let value = pai2[i];
        genes.swap(i, map[value]);

        let idx = map[value];
        map.swap(genes[idx], genes[i]);
    }

    genes
}

#[allow(dead_code)]
fn ordered_crossover<R: Rng + Sized>(mut rng: &mut R,
                                     pai1: &Sequencia,
                                     pai2: &Sequencia)
                                     -> Sequencia {
    let num_tarefas = pai1.len();

    let mut filho = vec![None; num_tarefas];
    let mut marcados = vec![false; num_tarefas];
    let (xbegin, xend) = gen_points(rng, num_tarefas);

    // Drop the swath
    for i in xbegin..xend {
        filho[i] = Some(pai1[i]);
        marcados[pai1[i]] = true;
    }

    let mut j = 0;
    let mut i = 0;
    while i < num_tarefas {
        if marcados[pai2[i]] {
            i += 1;
        } else if filho[j].is_some() {
            j += 1;
        } else {
            filho[j] = Some(pai2[i]);
            j += 1;
            i += 1;
        }
    }

    filho.into_iter().map(|o| o.expect("Erro no OX")).collect()
}

fn cruzamento<R: Rng + Sized>(mut rng: &mut R,
                              cruz: Cruzamento,
                              pais: Vec<(&Sequencia, &Sequencia)>)
                              -> Vec<Sequencia> {
    let mut filhos = Vec::with_capacity(2 * pais.len());
    let cruz_op = match cruz {
        Cruzamento::PMX => pmx_crossover,
        Cruzamento::OX => ordered_crossover,
    };

    for (pai1, pai2) in pais {
        filhos.push(cruz_op(rng, pai1, pai2));
        filhos.push(cruz_op(rng, pai2, pai1));
    }

    filhos
}

#[allow(dead_code)]
fn recombinacao<R: Rng + Sized>(mut rng: &mut R,
                                inst: &Instancia,
                                pais: Vec<(&Sequencia, &Sequencia)>,
                                mut_chance: f64,
                                cruz: Cruzamento,
                                metodo_mut: Mutacao)
                                -> Populacao {
    cruzamento(rng, cruz, pais)
        .into_iter()
        .map(|seq| mutacao(rng, seq, mut_chance, metodo_mut))
        .map(|seq| Solucao::new(inst, seq))
        .collect()
}

#[allow(dead_code)]
fn swap_vertices<R: Rng + Sized>(mut rng: &mut R, mut sequencia: Sequencia) -> Sequencia {
    let (i, j) = gen_points(rng, sequencia.len());
    sequencia.swap(i, j);
    sequencia
}

#[allow(dead_code)]
fn mutacao<R: Rng + Sized>(mut rng: &mut R,
                           sequencia: Sequencia,
                           mut_chance: f64,
                           metodo_mut: Mutacao)
                           -> Sequencia {
    let mut_op = match metodo_mut {
        Mutacao::Swap => swap_vertices,
        Mutacao::TwoOpt => two_opt_aleatorio,
    };
    if rng.gen::<f64>() < mut_chance {
        mut_op(rng, sequencia)
    } else {
        sequencia
    }
}

pub struct Ag<'a> {
    inst: &'a Instancia,
    timeout: u64,
    max_iter: u64,
    pop_tam: usize,
    xo_chance: f64,
    mut_chance: f64,
    cruz: Cruzamento,
    mutacao: Mutacao,
}
impl<'a> Ag<'a> {
    #[allow(dead_code)]
    pub fn new(inst: &Instancia) -> Ag {
        Ag {
            inst: inst,
            timeout: u64::MAX,
            max_iter: 1000,
            pop_tam: 200,
            xo_chance: 0.8,
            mut_chance: 0.1,
            cruz: Cruzamento::PMX,
            mutacao: Mutacao::Swap,
        }
    }

    #[allow(dead_code)]
    pub fn solve(&self) -> (Solucao, u64, u64) {
        solve(self.inst,
              Duration::from_secs(self.timeout),
              self.max_iter,
              self.pop_tam,
              self.xo_chance,
              self.cruz,
              self.mutacao,
              self.mut_chance)
    }

    #[allow(dead_code)]
    pub fn timeout(&mut self, timeout: u64) -> &mut Ag<'a> {
        self.timeout = timeout;
        self
    }

    #[allow(dead_code)]
    pub fn max_iter(&mut self, max_iter: u64) -> &mut Ag<'a> {
        self.max_iter = max_iter;
        self
    }

    #[allow(dead_code)]
    pub fn pop_tam(&mut self, pop_tam: usize) -> &mut Ag<'a> {
        self.pop_tam = pop_tam;
        self
    }

    #[allow(dead_code)]
    pub fn xo_chance(&mut self, xo_chance: f64) -> &mut Ag<'a> {
        self.xo_chance = xo_chance;
        self
    }

    #[allow(dead_code)]
    pub fn mut_chance(&mut self, mut_chance: f64) -> &mut Ag<'a> {
        self.mut_chance = mut_chance;
        self
    }

    #[allow(dead_code)]
    pub fn cruz(&mut self, cruz: Cruzamento) -> &mut Ag<'a> {
        self.cruz = cruz;
        self
    }

    #[allow(dead_code)]
    pub fn mutacao(&mut self, mutacao: Mutacao) -> &mut Ag<'a> {
        self.mutacao = mutacao;
        self
    }
}
