#! /usr/bin/env python3

from itertools import product
import sys

taxa_mut = [0.005, 0.010]
taxa_cruz = [0.95, 0.99]
tam_pop = [200, 400, 800]
operador_cruz = [0, 1]
operador_mut = [0, 1]
numero_grupos = 1

combinacoes = list(product(tam_pop, taxa_cruz, operador_cruz, operador_mut,
                           taxa_mut))
numero_combinacoes = len(combinacoes)
print('Numero de combinacoes: ', numero_combinacoes)

combinacoes_por_grupo = numero_combinacoes // numero_grupos
print('Combinacoes por grupo:', combinacoes_por_grupo)


def getid(c):
    return '.'.join(str(x) for x in c)


def comb2str(i, c):
    return 'ag ' + str(i) + '-' + getid(c) + ' ' + ' '.join(str(x) for x in c)


id_grupo = int(sys.argv[1])
comeco_intervalo = id_grupo * combinacoes_por_grupo
fim_intervalo = (id_grupo + 1) * combinacoes_por_grupo

print('Intervalo: [%d, %d)' % (comeco_intervalo, fim_intervalo))

with open('restantes.txt', 'w') as f:
    for i, c in enumerate(combinacoes[comeco_intervalo:fim_intervalo],
                          comeco_intervalo):
        print(comb2str(i, c), file=f)

