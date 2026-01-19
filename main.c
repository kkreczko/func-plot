#include "parser.h"

int main(int argc, char *argv[]) {
  Node expr = parseExpr(argv[1]);
  return 0;
}
