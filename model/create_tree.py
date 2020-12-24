from tree_to_code import *


def create_tree_native_main(file, clf, with_io, feature_set):
    tree_to_code(file, clf, clf.classes_, "decision_tree", feature_set, False)
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
    elif feature_set == 7:
        file.write("float args[9];\n")
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
        elif feature_set == 7:
            file.write("for (char i = 0; i < 9; ++i) sscanf(argv[i+1], \"%f\", &args[i]);\n")
    if feature_set == 6:
        file.write("return decision_tree(f_args, l_args);\n")
    else:
        file.write("return decision_tree(args);\n")
    file.write("}\n")


def create_tree_ino_evaluate(file, clf, feature_set):
    tree_to_code(file, clf, clf.classes_, "evaluate", feature_set, False)
