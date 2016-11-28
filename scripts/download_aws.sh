ips="
"
user="ubuntu"

for ip in $ips; do
  scp -i "$1" -r "$user"@"$ip":/home/"$user"/escalonamento/resultados .
done
