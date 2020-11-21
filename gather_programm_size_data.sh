# Creating a fresh start
if [[ -f "./size_data.csv" ]]; then
  rm ./size_data.csv
fi

touch ./size_data.csv

# We gather data for the following:
# Max_depth => 1 to 50
# Forest size => 1 to 64
# Optimization O0, Os, O2, O3

optimizations=(O0 Os O2 O3)

for max_depth in {1..50}; do
  for forest_size in {1..64}; do
    for opt in ${optimizations[*]}; do
      echo "Working on: ${max_depth},${forest_size},${opt}"
      python model/decision_tree.py $max_depth $forest_size 0 &> /dev/null
      ./../arduino-1.8.13/hardware/tools/avr/bin/avr-gcc -g -${opt} -std=gnu11 -ffunction-sections -fdata-sections -flto -fno-fat-lto-objects ./decision_tree.c -o decision_tree
      ./../arduino-1.8.13/hardware/tools/avr/bin/avr-gcc -g -${opt} -std=gnu11 -ffunction-sections -fdata-sections -flto -fno-fat-lto-objects ./decision_forest.c -o decision_forest
      size_in_bytes_tree=$(wc -c ./decision_tree | awk '{print $1}')
      size_in_bytes_forest=$(wc -c ./decision_forest | awk '{print $1}')
      if [[ -z "${size_in_bytes_tree}" ]]; then
        size_in_bytes_tree=-1
      fi
      if [[ -z "${size_in_bytes_forest}" ]]; then
        size_in_bytes_forest=-1
      fi
      echo "${max_depth},${forest_size},${opt},${size_in_bytes_tree},${size_in_bytes_forest}" >> ./size_data.csv
    done
  done
done

