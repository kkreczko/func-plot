#include "eval.h"
#include "parser.h"

#include <stdlib.h>

double evalExpr(Node *expression, double value) { 
  convertToLukasiewiczNotation(expression);
  return 10.0 * value; 
}

void convertToLukasiewiczNotation(Node *list) {
  Node operatorStack[STACK_SIZE];
  Node valueStack[STACK_SIZE];
  size_t opStackPtr = 0;
  size_t valStackPtr = 0;

  while (list) {
    if (isOperator(list)) {
      // operatorStack[opStackPtr] = malloc(sizeof(Node));
      operatorStack[opStackPtr] = *list;
      opStackPtr++;
    }

    if (list->token == TOK_Number) {
      // valueStack[valStackPtr] = malloc(sizeof(Node));
      valueStack[valStackPtr] = *list;
      valStackPtr++;
    }

    list = list->next;
  }
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
