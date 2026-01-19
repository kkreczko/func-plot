#ifndef PARSER_H
#define PARSER_H

// TODO think of a way to do trigonometry
// TODO Fast fourier transform? might be cool

typedef enum {
  TOK_Err,
  TOK_Plus,
  TOK_Minus,
  TOK_Multiply,
  TOK_Divide,
  TOK_Power,
  TOK_End,
  TOK_ParenOpen,
  TOK_ParenClose,
  TOK_Var,
  TOK_Number,
  TOK_WhiteSpace,
} Token;

typedef struct Node {
  double value;
  Token token;
  char *name;
  struct Node *next;
} Node;

Node parseExpr(char *expr);

// TODO later, too lazy, trust
int verifyExpr(Node expr);

// replace variable in expression with value for quick maths
double evalExpr(Node expr);

void addItem(Node *list, Node *item);

void dump(Node *list);

#endif
