# Escalonamento

## Implementação de algoritmos para resolução de Escalonamento de Máquina Única em Rust

### Algoritmos implementados
- GRASP:
    - Construção: earliest due date semi-guloso
    - Busca local: best-improvement hill climbing com 2-opt
- AG:
    - População inicial: sequências aleatórias
    - Seleção: roleta simples
    - Cruzamento: PMX e OX
    - Mutação: 2-opt aleatório e swap
    - Próxima geração: elitismo