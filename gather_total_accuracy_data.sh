# Creating a fresh start
if [[ -f "./accuracy_data.csv" ]]; then
  rm ./accuracy_data.csv
fi

echo "max_depth,forest_size,accuracy" > ./accuracy_data.csv

# We gather data for the following:
# Max_depth => 1 to 50
# Forest size => 1 to 64

for max_depth in {1..30}; do
  for forest_size in {1..32}; do
    echo "Working on: ${max_depth},${forest_size}..."
    python model/decision_tree.py $max_depth $forest_size 1 &> /dev/null
      gcc -O2 ./decision_forest.c -o decision_forest
      accuracy=$(DATA_PATH=".." cargo test test_kubik_test_by_annotation_decision_forest --bin simulation -- --nocapture | grep "Total accuracy:" | grep -o -E "[0-9]\.[0-9]+")
      echo "${max_depth},${forest_size},${accuracy}" >> ./accuracy_data.csv
    done
  done
done

