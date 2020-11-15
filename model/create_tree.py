from tree_to_code import *


def create_tree_native_main(file, clf):
    tree_to_code(file, clf, "decision_tree")
    file.write("#include <stdio.h>\n")
    file.write("int main(int argc, char** argv) {\n")
    # file.write("short int args[27];\n")
    # file.write("for (char i = 0; i < 27; ++i) sscanf(argv[i+1], \"%hd\", &args[i]);\n")
    file.write("float args[12];\n")
    file.write("for (char i = 0; i < 12; ++i) sscanf(argv[i+1], \"%f\", &args[i]);\n")
    file.write("return decision_tree(args);\n")
    # file.write("printf(\"Result: %d\\n\", (int) result);\n")
    file.write("}\n")


def create_tree_ino_evaluate(file, clf):
    tree_to_code(file, clf, "evaluate")