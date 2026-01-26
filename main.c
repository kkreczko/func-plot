#include "eval.h"
#include "parser.h"

#include <stdio.h>

int main(int argc, char *argv[]) {
  Node expr = parseExpr(argv[1]);
  dump(&expr);
  double someResult = evalExpr(&expr, 10.0);
  printf("%f\n", someResult);
  return 0;
}
