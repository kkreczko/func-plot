#ifndef PARSER_H
#define PARSER_H

#define NODE_BUFF 128

typedef enum {
  TOK_Err,
  TOK_Plus,
  TOK_Minus,
  TOK_Multiply,
  TOK_Divide,
  TOK_Power,
  TOK_Start,
  TOK_End,
  TOK_ParenOpen,
  TOK_ParenClose,
  TOK_Var,
  TOK_Number,
  TOK_Pi,
  TOK_Sin,
  TOK_Cos,
  TOK_Euler,
  TOK_Test,
} Token;

// TODO switch current linked list approach in parsing to node stack
typedef struct Node {
  double value;
  Token token;
  char *name;
  struct Node *next;
} Node;

void dump(Node *list);
void clearMem(Node *list);

Node parseExpr(char *expr);
int verifyExpr(Node expr);

int isReserved(char name);
void addItem(Node *list, Node *item);

void combineNumbers(Node *list);
void removeRedundantNumbers(Node *expr);
double calculateValue(double *nodeValues, int exponent);

#endif
