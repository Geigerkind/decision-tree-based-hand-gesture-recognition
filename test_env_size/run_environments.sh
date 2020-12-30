min_leaf_samples=(1 2 4 8 16 32 64 128 256 512 1024 2048 4096)
num_cores_per_node=3

i=0
for min_leaf_sample in ${min_leaf_samples[*]}; do
  # Delete old
  rm -r env_${i} &> /dev/null

  # Create new environment
  mkdir env_${i}
  cp ./gather_size_and_accuracy.sh env_${i}/

  # Execute it as job
  cd env_${i}
  nice -n 19 bash ./gather_size_and_accuracy.sh 0.0 ${min_leaf_sample} ${num_cores_per_node} ${i} &>> log.txt &
  cd ../

  # Prepare next job
  ((i=i+1))
done

# Wait for all jobs to finish
wait $(jobs -p)