from tree_to_code import *


def create_forest(file, trees, num_trees, feature_set):
    for i in range(num_trees):
        tree_to_code(file, trees[i], 0, "tree" + str(i), feature_set, True)
        file.write("\n")


def shared_forest_print_stuff(file, feature_set, classes, num_trees):
    file.write("float tree_res[5] = { 0.0, 0.0, 0.0, 0.0, 0.0 };\n")
    file.write("float total_res[5] = { 0.0, 0.0, 0.0, 0.0, 0.0 };\n")
    if len(classes) == 4:
        file.write("unsigned char result_map[5] = { " + str(classes[0]) + "," + str(classes[1]) + "," + str(
            classes[2]) + "," + str(classes[3]) + ", 9 };\n")
    else:
        file.write("unsigned char result_map[5] = { " + str(classes[0]) + "," + str(classes[1]) + "," + str(
            classes[2]) + "," + str(classes[3]) + ", " + str(classes[4])+" };\n")
    for i in range(num_trees):
        if feature_set == 6:
            file.write("tree" + str(i) + "(f_args, l_args, tree_res);\n")
        else:
            file.write("tree" + str(i) + "(args, tree_res);\n")
        file.write("total_res[0] += tree_res[0];\n")
        file.write("total_res[1] += tree_res[1];\n")
        file.write("total_res[2] += tree_res[2];\n")
        file.write("total_res[3] += tree_res[3];\n")
        file.write("total_res[4] += tree_res[4];\n")

    file.write("unsigned char max_index = 0;\n")
    file.write("float max_value = 0;\n")
    file.write("for (unsigned char i = 0; i < 5; ++i) {\n")
    file.write("if (max_value < total_res[i]) {\n")
    file.write("max_value = total_res[i];\n")
    file.write("max_index = i;\n")
    file.write("}\n")
    file.write("}\n")
    file.write("return result_map[max_index];\n")
    file.write("}\n")

def create_forest_native_main(file, trees, classes, num_trees, with_io, feature_set):
    create_forest(file, trees, num_trees, feature_set)
    if with_io:
        file.write("#include <stdio.h>\n")
    file.write("int main(int argc, char** argv) {\n")
    if feature_set == 1:
        file.write("float args[10];\n")
    elif feature_set == 2:
        file.write("long args[10];\n")
    elif feature_set == 3:
        file.write("long args[9];\n")
    elif feature_set == 4:
        file.write("long args[12];\n")
    elif feature_set == 5:
        file.write("long args[21];\n")
    elif feature_set == 6:
        file.write("float f_args[10];\n")
        file.write("long l_args[10];\n")

    if with_io:
        if feature_set == 1:
            file.write("for (char i = 0; i < 10; ++i) sscanf(argv[i+1], \"%f\", &args[i]);\n")
        elif feature_set == 2:
            file.write("for (char i = 0; i < 10; ++i) sscanf(argv[i+1], \"%ld\", &args[i]);\n")
        elif feature_set == 3:
            file.write("for (char i = 0; i < 9; ++i) sscanf(argv[i+1], \"%ld\", &args[i]);\n")
        elif feature_set == 4:
            file.write("for (char i = 0; i < 12; ++i) sscanf(argv[i+1], \"%ld\", &args[i]);\n")
        elif feature_set == 5:
            file.write("for (char i = 0; i < 21; ++i) sscanf(argv[i+1], \"%ld\", &args[i]);\n")
        elif feature_set == 6:
            file.write("for (char i = 0; i < 10; ++i) sscanf(argv[i+1], \"%f\", &f_args[i]);\n")
            file.write("for (char i = 0; i < 10; ++i) sscanf(argv[i+1 + 10], \"%ld\", &l_args[i]);\n")

    shared_forest_print_stuff(file, feature_set, classes, num_trees)


def create_forest_ino_evaluate(file, trees, classes, num_trees, feature_set):
    create_forest(file, trees, num_trees, feature_set)
    if feature_set == 1:
        file.write("unsigned char evaluate_forest(float* args) {\n")
    elif feature_set == 6:
        file.write("unsigned char evaluate_forest(float* f_args, long* l_args) {\n")
    else:
        file.write("unsigned char evaluate_forest(long* args) {\n")
    shared_forest_print_stuff(file, feature_set, classes, num_trees)
