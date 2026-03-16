#ifndef EVAL_H
#define EVAL_H

#include "parser.h"

#define STACK_SIZE 169

double evalExpr(Node *expression, double value);
double evalSubExpr(Node *list);

#endif
