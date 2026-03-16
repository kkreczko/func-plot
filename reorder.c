#include "reorder.h"

void reorderToPolishNotation(Node *result) { return; }

int getOperatorPriority(Node *item) { return 1; }

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

int isNumOrVar(Node *item) { return 1; }

int isEnd(Node *item) { return 1; }
