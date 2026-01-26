#include "eval.h"
#include "parser.h"

#include <stdlib.h>

double evalExpr(Node *list) { return 1.0; }

void convertToLukasiewiczNotation(Node *list) {
  Node operatorStack[STACK_SIZE];
  Node valueStack[STACK_SIZE];
}

int isOperator(Token item) {
  switch (item) {
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
