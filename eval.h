#ifndef EVAL_H
#define EVAL_H

#include "parser.h"

#define STACK_SIZE 169

// polish notation correct name
void convertToLukasiewiczNotation(Node *list);

double evalExpr(Node *expression, double value);
double evalSubExpr(Node *list);
int isOperator(Node *item);

#endif
