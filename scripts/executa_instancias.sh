for ((i=1; i<=5; i++)); do
    nohup python3 ../scripts/executa_experimentos.py ../instances/1_50_1_$i -e &
done