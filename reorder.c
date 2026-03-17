#include "reorder.h"
#include "parser.h"

void reorderToPolishNotation(Node *result) { return; }

int getOperatorPriority(Node *item) {
  switch (item->token) {
  case TOK_ParenOpen:
  case TOK_ParenClose:
    return 99;
  case TOK_Power:
    return 87;
  case TOK_Multiply:
  case TOK_Divide:
    return 69;
  case TOK_Plus:
  case TOK_Minus:
    return 42;
  default:
    return 0;
  }
}

int isOperator(Node *item) {
  switch (item->token) {
  case TOK_Plus:
  case TOK_Minus:
  case TOK_Divide:
  case TOK_Multiply:
  case TOK_Power:
  case TOK_ParenOpen:
  case TOK_ParenClose:
    return 1;
  default:
    return 0;
  }
}

int isNumOrVar(Node *item) {
  Token token = item->token;
  return (token == TOK_Number || token == TOK_Var);
}

int isEnd(Node *item) { return item->token == TOK_End; }
