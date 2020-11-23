# Creating a fresh start
if [[ -f "./size_data.csv" ]]; then
  rm ./size_data.csv
fi

echo "max_depth,forest_size,optimization_level,tree_bytes,forest_bytes" > ./size_data.csv

# We gather data for the following:
# Max_depth => 1 to 50
# Forest size => 1 to 64
# Optimization O0, Os, O2, O3

optimizations=(O0 Os O2 O3)

for max_depth in {1..30}; do
  for forest_size in {1..32}; do
    echo "Working on: ${max_depth},${forest_size}..."
    python model/decision_tree.py $max_depth $forest_size 1 &> /dev/null
    for opt in ${optimizations[*]}; do
      echo "${opt}"
      #./../arduino-1.8.13/hardware/tools/avr/bin/avr-gcc -${opt} -std=gnu11 -mmcu=atmega328p -g -flto -fuse-linker-plugin -Wl,--gc-sections ./decision_tree.c -o decision_tree
      ./../arduino-1.8.13/hardware/tools/avr/bin/avr-gcc -${opt} -std=gnu11 -mmcu=atmega328p -s -fwhole-program -fdata-sections -ffunction-sections -flto -fuse-linker-plugin -Wl,--gc-sections -Wl,--strip-all ./decision_tree.c -o decision_tree
      #./../arduino-1.8.13/hardware/tools/avr/bin/avr-gcc -${opt} -std=gnu11 -mmcu=atmega328p -g -flto -fuse-linker-plugin -Wl,--gc-sections ./decision_forest.c -o decision_forest
      ./../arduino-1.8.13/hardware/tools/avr/bin/avr-gcc -${opt} -std=gnu11 -mmcu=atmega328p -s -fwhole-program -fdata-sections -ffunction-sections -flto -fuse-linker-plugin -Wl,--gc-sections -Wl,--strip-all ./decision_forest.c -o decision_forest
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

