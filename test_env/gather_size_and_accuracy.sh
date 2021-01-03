# Creating a fresh start
if [[ -f "./size_and_accuracy_data.csv" ]]; then
  rm ./size_and_accuracy_data.csv
fi

echo "max_depth,forest_size,optimization_level,ensemble_technique,feature_set,set_fraction,ccp_alpha,min_leaf_sample,forest_bytes,accuracy_klisch,accuracy_dymel_null,accuracy_dymel_gesture" > ./size_and_accuracy_data.csv

# We gather data for the following:
# Max_depth => 1 to 30
# Forest size => 1 to 20
# Optimization O0, Os, O2, O3

optimizations=(O0 Os O2 O3)
set_fractions=(0.5)
feature_sets=(1 2 4 7)

ccp_alpha=$1
min_leaf_sample=$2
num_cores_per_node=$3
env_num=$4

for feature_set in ${feature_sets[*]}; do
  for set_fraction in ${set_fractions[*]}; do
    for ensemble_technique in {3..3}; do
      for max_depth in {1..22}; do
        for forest_size in {1..16}; do
          echo "Working on: ${max_depth},${forest_size},${ensemble_technique},${feature_set},${set_fraction},${ccp_alpha},${min_leaf_sample}..."
          python3 ../../model/decision_tree.py ${max_depth} ${forest_size} 1 ${ensemble_technique} 1 ${feature_set} ${set_fraction} ${ccp_alpha} ${min_leaf_sample} 1 "../../" ${num_cores_per_node} 0
          real_max_depth=$?
          gcc -O2 ./decision_forest.c -o decision_forest
          accuracy_klisch=$(DATA_PATH=".." PROGRAM_PATH="./../test_env/env_${env_num}" cargo test test_klisch_test_by_annotation_decision_forest --target-dir "./" --bin simulation --manifest-path "../../simulation/Cargo.toml" --features "feature_set${feature_set}" -- --nocapture | grep "Total accuracy:" | grep -o -E "[0-9]\.[0-9]+")
          accuracy_dymel_null=$(DATA_PATH=".." PROGRAM_PATH="./../test_env/env_${env_num}" cargo test test_dymel_test_null --bin simulation --target-dir "./" --manifest-path "../../simulation/Cargo.toml" --features "feature_set${feature_set}" -- --nocapture | grep "Total accuracy:" | grep -o -E "[0-9]\.[0-9]+")
          accuracy_dymel_gesture=$(DATA_PATH=".." PROGRAM_PATH="./../test_env/env_${env_num}" cargo test test_dymel_test_gesture --bin simulation --target-dir "./" --manifest-path "../../simulation/Cargo.toml" --features "feature_set${feature_set}" -- --nocapture | grep "Total accuracy:" | grep -o -E "[0-9]\.[0-9]+")
          for opt in ${optimizations[*]}; do
            echo "${opt}"
            ./../../../arduino-1.8.13/hardware/tools/avr/bin/avr-gcc -${opt} -std=gnu11 -mmcu=atmega328p -s -fwhole-program -fdata-sections -ffunction-sections -flto -fuse-linker-plugin -Wl,--gc-sections -Wl,--strip-all ./decision_forest.c -o decision_forest_avr
            size_in_bytes_forest=$(wc -c ./decision_forest_avr | awk '{print $1}')
            if [[ -z "${size_in_bytes_forest}" ]]; then
              size_in_bytes_forest=-1
            fi
            echo "${real_max_depth},${forest_size},${opt},${ensemble_technique},${feature_set},${set_fraction},${ccp_alpha},${min_leaf_sample},${size_in_bytes_forest},${accuracy_klisch},${accuracy_dymel_null},${accuracy_dymel_gesture}" >> ./size_and_accuracy_data.csv
          done
        done
      done
    done
  done
done
