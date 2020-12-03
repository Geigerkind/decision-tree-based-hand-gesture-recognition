
ccp_alphas=(0.0 0.001 0.01 0.1)
min_leaf_samples=(1 2 4 8)
num_cores_per_node=1

i=0
for ccp_alpha in ${ccp_alphas[*]}; do
  for min_leaf_sample in ${min_leaf_samples[*]}; do
    # Delete old
    rm -r env_${i} &> /dev/null

    # Create new environment
    mkdir env_${i}
    cp ./gather_size_and_accuracy.sh env_${i}/

    # Execute it as job
    cd env_${i}
    nice -n 19 bash ./gather_size_and_accuracy.sh ${ccp_alpha} ${min_leaf_sample} ${num_cores_per_node} ${i} &>> log.txt &
    cd ../

    # Prepare next job
    ((i=i+1))
  done
done

# Wait for all jobs to finish
wait $(jobs -p)