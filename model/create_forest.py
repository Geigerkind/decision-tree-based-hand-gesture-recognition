from tree_to_code import *


def create_forest(file, trees, classes, num_trees, is_float):
    for i in range(num_trees):
        tree_to_code(file, trees[i], classes, "tree" + str(i), is_float)
        file.write("\n")


def create_forest_native_main(file, trees, classes, num_trees, with_io, is_float):
    create_forest(file, trees, classes, num_trees, is_float)
    if with_io:
        file.write("#include <stdio.h>\n")
    file.write("int main(int argc, char** argv) {\n")
    if is_float:
        file.write("float args[10];\n")
    else:
        file.write("long args[10];\n")
    file.write("unsigned int results[10] = { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };\n")
    if with_io:
        if is_float:
            file.write("for (unsigned char i = 0; i < 10; ++i) sscanf(argv[i+1], \"%f\", &args[i]);\n")
        else:
            file.write("for (unsigned char i = 0; i < 10; ++i) sscanf(argv[i+1], \"%ld\", &args[i]);\n")
    for i in range(num_trees):
        file.write("++results[tree" + str(i) + "(args)];\n")
    file.write("unsigned char max_index = 0;\n")
    file.write("unsigned int max_value = 0;\n")
    file.write("for (unsigned char i = 0; i < 10; ++i) {\n")
    file.write("if (max_value < results[i]) {\n")
    file.write("max_value = results[i];\n")
    file.write("max_index = i;\n")
    file.write("}\n")
    file.write("}\n")
    file.write("return max_index;\n")
    file.write("}\n")


def create_forest_ino_evaluate(file, trees, classes, num_trees, is_float):
    create_forest(file, trees, classes, num_trees, is_float)
    if is_float:
        file.write("unsigned char evaluate_forest(float* args) {\n")
    else:
        file.write("unsigned char evaluate_forest(long* args) {\n")
    file.write("unsigned int results[10] = { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };\n")
    for i in range(num_trees):
        file.write("++results[tree" + str(i) + "(args)];\n")
    file.write("unsigned char max_index = 1;\n")
    file.write("unsigned int max_value = results[1];\n")
    file.write("if (max_value < results[2]) {\n")
    file.write("max_value = results[2];\n")
    file.write("max_index = 2;\n")
    file.write("}\n")
    file.write("if (max_value < results[3]) {\n")
    file.write("max_value = results[3];\n")
    file.write("max_index = 3;\n")
    file.write("}\n")
    file.write("if (max_value < results[4]) {\n")
    file.write("max_value = results[4];\n")
    file.write("max_index = 4;\n")
    file.write("}\n")
    file.write("if (max_value < results[9]) {\n")
    file.write("max_value = results[9];\n")
    file.write("max_index = 9;\n")
    file.write("}\n")
    file.write("return max_index;\n")
    file.write("}\n")