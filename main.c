#include "eval.h"
#include "parser.h"

#include <stdio.h>

int main(int argc, char *argv[]) {
  Node expr = parseExpr(argv[1]);
  dump(&expr);

  return 0;
}
