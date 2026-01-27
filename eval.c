#include "eval.h"
#include "parser.h"

#include <stdlib.h>

double evalExpr(Node *expression, double value) { 
  return 10.0 * value; 
}

int isOperator(Node *item) {
  switch (item->token) {
  case TOK_Plus:
    return 1;
  case TOK_Minus:
    return 1;
  case TOK_Divide:
    return 1;
  case TOK_Multiply:
    return 1;
  case TOK_Power:
    return 1;
  default:
    return 0;
  }
}
