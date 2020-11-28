# Creating a fresh start
if [[ -f "./size_and_accuracy_data.csv" ]]; then
  rm ./size_and_accuracy_data.csv
fi

echo "max_depth,forest_size,optimization_level,ensamble_technique,feature_set,set_fraction,forest_bytes,accuracy" > ./size_and_accuracy_data.csv

# We gather data for the following:
# Max_depth => 1 to 30
# Forest size => 1 to 20
# Optimization O0, Os, O2, O3

optimizations=(O0 Os O2 O3)
set_fractions=(0.05 0.1 0.15 0.2 0.25 0.3 0.35 0.4 0.45 0.5)

for feature_set in {1..2}; do
  for set_fraction in ${set_fractions[*]}; do
    for ensamble_technique in {1..4}; do
      for max_depth in {1..25}; do
        for forest_size in {1..22}; do
          echo "Working on: ${max_depth},${forest_size},${ensamble_technique},${feature_set},${set_fraction},..."
          python model/decision_tree.py $max_depth $forest_size 1 ${ensamble_technique} 1 ${feature_set} &> /dev/null
          gcc -O2 ./decision_forest.c -o decision_forest
          accuracy=$(DATA_PATH=".." cargo test test_klisch_test_by_annotation_decision_forest --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set${feature_set}" -- --nocapture | grep "Total accuracy:" | grep -o -E "[0-9]\.[0-9]+")
          for opt in ${optimizations[*]}; do
            echo "${opt}"
            ./../arduino-1.8.13/hardware/tools/avr/bin/avr-gcc -${opt} -std=gnu11 -mmcu=atmega328p -s -fwhole-program -fdata-sections -ffunction-sections -flto -fuse-linker-plugin -Wl,--gc-sections -Wl,--strip-all ./decision_forest.c -o decision_forest
            size_in_bytes_forest=$(wc -c ./decision_forest | awk '{print $1}')
            if [[ -z "${size_in_bytes_forest}" ]]; then
              size_in_bytes_forest=-1
            fi
            echo "${max_depth},${forest_size},${opt},${ensamble_technique},${feature_set},${set_fraction},${size_in_bytes_forest},${accuracy}" >> ./size_and_accuracy_data.csv
          done
        done
      done
    done
  done
done
