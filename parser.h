#ifndef PARSER_H
#define PARSER_H

typedef enum {
  TOK_Err,
  TOK_Plus,
  TOK_Minus,
  TOK_Multiply,
  TOK_Divide,
  TOK_End,
  TOK_ParenOpen,
  TOK_ParenClose,
  TOK_Var
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

// replace operator in expression with value for quick maths
double evalExpr(Node expr[], double value);

#endif
