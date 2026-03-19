#include "reorder.h"
#include "parser.h"

void reorderToPolishNotation(Node *result) { return; }

int getOperatorPriority(Node *item) {
  switch (item->token) {
  case TOK_Power:
    return 2137;
  case TOK_Multiply:
  case TOK_Divide:
    return 420;
  case TOK_Plus:
  case TOK_Minus:
    return 69;
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
    return 1;
  default:
    return 0;
  }
}

int isParen(Node *item) {
  Token token = item->token;
  return (token == TOK_ParenOpen || token == TOK_ParenClose);
}

int isNumOrVar(Node *item) {
  Token token = item->token;
  return (token == TOK_Number || token == TOK_Var);
}

int isEnd(Node *item) { return item->token == TOK_End; }
