extern crate rand;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};
use std::path::Path;
use std::fs::File;
use std::cmp::max;
use self::rand::Rng;

pub const INF: i32 = 1e9 as i32;

pub static mut CHAMADAS_FO: u64 = 0;

pub type IdTarefa = usize;
pub type Sequencia = Vec<IdTarefa>;

pub struct Tarefa {
    duracao: i32,
    entrega: i32,
}

impl Tarefa {
    pub fn duracao(&self) -> i32 {
        self.duracao
    }

    pub fn entrega(&self) -> i32 {
        self.entrega
    }

    pub fn new(duracao: i32, entrega: i32) -> Tarefa {
        Tarefa {
            duracao: duracao,
            entrega: entrega,
        }
    }
}

pub struct Instancia(Vec<Tarefa>);

impl Instancia {
    pub fn num_tarefas(&self) -> usize {
        self.0.len()
    }

    pub fn tarefa(&self, id: IdTarefa) -> &Tarefa {
        &self.0[id]
    }

    pub fn toy() -> Instancia {
        Instancia(vec![])
    }

    pub fn from_arquivo(file: &str) -> Instancia {
        let path = Path::new(file);
        let mut file = BufReader::new(File::open(&path).expect("Erro ao abrir arquivo"));

        let mut num_line = String::new();
        file.read_line(&mut num_line).unwrap();
        let n: usize = num_line.trim().parse().expect("Erro ao ler o número de tarefas");

        Instancia(file.lines()
            .take(n)
            .map(|l| {
                l.expect("Erro ao ler linha")
                    .split_whitespace()
                    .map(|number| number.parse().expect("Erro ao ler dado da tarefa"))
                    .collect::<Vec<i32>>()
            })
            .map(|val| Tarefa::new(val[0], val[1]))
            .collect())
    }
}


#[derive(Clone)]
pub struct Solucao {
    sequencia: Sequencia,
    fo: i32,
}

fn is_factivel(s: &Sequencia, num_tarefas: usize) -> bool {
    s.len() == num_tarefas && frequencias(s).into_iter().all(|n| n == 1)
}

fn frequencias(sequencia: &Sequencia) -> Vec<u64> {
    let mut freq = vec![0; sequencia.len()];
    for &vert in sequencia {
        freq[vert] += 1;
    }
    freq
}

impl Solucao {
    fn calcula_fo(inst: &Instancia, sequencia: &Sequencia) -> i32 {
        unsafe {
            CHAMADAS_FO += 1;
        }

        if !is_factivel(sequencia, inst.num_tarefas()) {
            return INF;
        }
        let tempo_finalizacao = Solucao::executar_simulacao(inst, sequencia);
        sequencia.iter()
            .map(|&id| {
                let t = inst.tarefa(id);
                let atraso = max(0, tempo_finalizacao[id] - t.entrega());
                let adiantamento = max(0, t.entrega() - tempo_finalizacao[id]);
                adiantamento + atraso
            })
            .sum()
    }

    fn executar_simulacao(inst: &Instancia, sequencia: &Sequencia) -> Vec<i32> {
        let mut finalizacao = vec![0; inst.num_tarefas()];
        let mut t_atual = 0;
        for &id in sequencia {
            t_atual += inst.tarefa(id).duracao();
            finalizacao[id] = t_atual;
        }
        finalizacao
    }

    pub fn new(inst: &Instancia, sequencia: Sequencia) -> Solucao {
        Solucao {
            fo: Solucao::calcula_fo(inst, &sequencia),
            sequencia: sequencia,
        }
    }

    #[allow(dead_code)]
    pub fn vazia() -> Solucao {
        Solucao {
            fo: INF,
            sequencia: vec![],
        }
    }

    pub fn sequencia(&self) -> &Sequencia {
        &self.sequencia
    }

    pub fn fo(&self) -> i32 {
        self.fo
    }
}

#[allow(dead_code)]
pub fn neh(inst: &Instancia) -> Solucao {
    let mut sol = Solucao::new(&inst, vec![]);
    let n = inst.num_tarefas();
    let mut seq: Vec<_> = (0..n).collect();
    seq.sort_by_key(|t| -inst.tarefa(*t).entrega());

    while !seq.is_empty() {
        let t = seq.pop().unwrap();
        let mut best: Option<Solucao> = None;
        for i in 0..sol.sequencia().len() {
            let mut v = sol.sequencia().clone();
            v.insert(i, t);
            let v = Solucao::new(&inst, v);
            if best.is_none() || v.fo() < best.as_ref().unwrap().fo() {
                best = Some(v);
            }
        }
        sol = best.unwrap();
    }

    sol
}

pub fn busca(inst: &Instancia) -> Solucao {
    let n = inst.num_tarefas();
    let mut s: Vec<_> = (0..n).collect();
    s.sort_by_key(|t| inst.tarefa(*t).entrega() - inst.tarefa(*t).duracao());
    let mut s = Solucao::new(&inst, s);
    let mut rng = rand::weak_rng();

    let t = Instant::now();
    let timeout = Duration::from_secs(5);

    while t.elapsed() < timeout {
        let t = rng.gen::<IdTarefa>() % n;
        for j in 0..n {
            let mut v = s.sequencia().clone();
            v.retain(|&x| x != t);
            v.insert(j, t);
            let v = Solucao::new(&inst, v);
            if v.fo() < s.fo() {
                s = v;
            }
        }
    }

    s
}
