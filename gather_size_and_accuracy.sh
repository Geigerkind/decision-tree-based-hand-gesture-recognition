# Creating a fresh start
if [[ -f "./size_and_accuracy_data.csv" ]]; then
  rm ./size_and_accuracy_data.csv
fi

echo "max_depth,forest_size,optimization_level,forest_bytes,accuracy" > ./size_and_accuracy_data.csv

# We gather data for the following:
# Max_depth => 1 to 30
# Forest size => 1 to 20
# Optimization O0, Os, O2, O3

optimizations=(O0 Os O2 O3)

for max_depth in {12..20}; do
  for forest_size in {1..10}; do
    echo "Working on: ${max_depth},${forest_size}..."
    python model/decision_tree.py $max_depth $forest_size 1 &> /dev/null
    gcc -O2 ./decision_forest.c -o decision_forest
    accuracy=$(DATA_PATH=".." cargo test test_klisch_test_by_annotation_decision_forest --bin simulation -- --nocapture | grep "Total accuracy:" | grep -o -E "[0-9]\.[0-9]+")
    for opt in ${optimizations[*]}; do
      echo "${opt}"
      ./../arduino-1.8.13/hardware/tools/avr/bin/avr-gcc -${opt} -std=gnu11 -mmcu=atmega328p -s -fwhole-program -fdata-sections -ffunction-sections -flto -fuse-linker-plugin -Wl,--gc-sections -Wl,--strip-all ./decision_forest.c -o decision_forest
      size_in_bytes_forest=$(wc -c ./decision_forest | awk '{print $1}')
      if [[ -z "${size_in_bytes_forest}" ]]; then
        size_in_bytes_forest=-1
      fi
      echo "${max_depth},${forest_size},${opt},${size_in_bytes_forest},${accuracy}" >> ./size_and_accuracy_data.csv
    done
  done
done

