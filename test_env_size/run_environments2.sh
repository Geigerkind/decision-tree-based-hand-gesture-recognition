ccp_alphas=(0.0 0.0010165625 0.002033125 0.00406625 0.0081325 0.016265 0.03125 0.0625 0.125 0.25 0.5)
num_cores_per_node=3

i=0
for ccp_alpha in ${ccp_alphas[*]}; do
  # Delete old
  rm -r env_${i} &> /dev/null

  # Create new environment
  mkdir env_${i}
  cp ./gather_size_and_accuracy.sh env_${i}/

  # Execute it as job
  cd env_${i}
  nice -n 19 bash ./gather_size_and_accuracy.sh ${ccp_alpha} 1 ${num_cores_per_node} ${i} &>> log.txt &
  cd ../

  # Prepare next job
  ((i=i+1))
done

# Wait for all jobs to finish
wait $(jobs -p)