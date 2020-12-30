# Creating a fresh start
if [[ -f "./size_data.csv" ]]; then
  rm ./size_data.csv
fi

echo "max_depth,ccp_alpha,min_leaf_sample,tree_bytes" > ./size_data.csv

ccp_alpha=$1
min_leaf_sample=$2
num_cores_per_node=$3
env_num=$4

echo "Working on: ${ccp_alpha},${min_leaf_sample}..."
python3 ../../model/decision_tree.py 16 1 1 1 0 1 0.5 ${ccp_alpha} ${min_leaf_sample} 1 "../../" ${num_cores_per_node}
real_max_depth=$?
./../../../arduino-1.8.13/hardware/tools/avr/bin/avr-gcc -Os -std=gnu11 -mmcu=atmega328p -s -fwhole-program -fdata-sections -ffunction-sections -flto -fuse-linker-plugin -Wl,--gc-sections -Wl,--strip-all ./decision_tree.c -o decision_tree_avr
size_in_bytes=$(wc -c ./decision_tree_avr | awk '{print $1}')
if [[ -z "${size_in_bytes}" ]]; then
  size_in_bytes=-1
fi
echo "${real_max_depth},${ccp_alpha},${min_leaf_sample},${size_in_bytes}" >> ./size_data.csv
