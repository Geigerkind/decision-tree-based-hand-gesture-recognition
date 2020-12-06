
cp env_0/size_and_accuracy_data.csv ./result.csv
for i in {1..7}; do
  awk FNR-1 env_${i}/size_and_accuracy_data.csv >> ./result.csv
done