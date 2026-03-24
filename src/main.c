#include "parser.h"
#include "stdlib.h"

int main(int argc, char *argv[]) {
  Node result = parseExpr(argv[1]);
  // reorder to rpm -> evalExpr(result, 10);
  dump(&result);
  // clearMem();
  return 0;
}
