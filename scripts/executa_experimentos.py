#! /usr/bin/env python3

import atexit
import os
from subprocess import PIPE, run, STDOUT, CalledProcessError

exe = "cargo run --release -- "
restantes_file = "restantes.txt"
finalizados_file = "finalizados.txt"
result_folder = "resultados"
out_folder = os.path.join(os.path.curdir, result_folder)

restantes = []
finalizados = []


def escreve_arquivos():
    with open(finalizados_file, 'w') as f:
        s = '\n'.join(finalizados)
        print(s, file=f, end='')

    with open(restantes_file, 'w') as f:
        s = '\n'.join(restantes)
        print(s, file=f, end='')

atexit.register(escreve_arquivos)

with open(restantes_file) as f:
    restantes = [l.strip() for l in f if l.strip()]

try:
    with open(finalizados_file) as f:
        finalizados = [l.strip() for l in f if l.strip()]
except FileNotFoundError:
    pass

os.makedirs(out_folder, exist_ok=True)
print('Pasta de saida:', out_folder, '\n')

while restantes:
    c = restantes[-1]
    print('Config:', c)

    infile = "TODO"

    p = run([exe, infile, '-e'], input=c, stdout=PIPE, stderr=STDOUT,
            universal_newlines=True)

    try:
        p.check_returncode()
    except CalledProcessError as e:
        print('Erro ao executar o processo, código: {}, mensagem: {}'
              .format(e.returncode, e.stderr))

    restantes.pop()
    finalizados.append(c)

    aid = c.split()[0]
    saida = p.stdout
    print(aid, '\n', saida, end='\n\n')

    out_path = os.path.join(out_folder, aid + '.csv')
    with open(out_path, 'w') as f:
        print(saida, file=f, end='')

