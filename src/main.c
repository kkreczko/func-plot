#include "parser.h"

int main(int argc, char *argv[]) {
  Node result = parseExpr(argv[1]);
  // reorder to rpm -> evalExpr(result, 10);
  dump(&result);
  return 0;
}
