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
  TOK_Number
} Token;

typedef struct {
  double value;
  Token token;
  char name[1];
} Node;

typedef struct {
  Node curr;
  Node next;
} NodeList;

NodeList parseExpr(char expr[]);

// TODO later, too lazy, trust
int verifyExpr(NodeList expr);

// replace variable in expression with value for quick maths
double evalExpr(NodeList expr);

#endif
