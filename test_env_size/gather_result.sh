
cp env_0/size_data.csv ./result.csv
for i in {1..10}; do
  awk FNR-1 env_${i}/size_data.csv >> ./result.csv
done