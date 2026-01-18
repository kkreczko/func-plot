#include "parser.h"

int main(int argc, char *argv[]) {
  NodeList expr = parseExpr(argv[1]);
  return 0;
}
