#ifndef EVAL_H
#define EVAL_H

#include "parser.h"

double evalExpr(Node *expression, double value);
double evalSubExpr(Node *list);

#endif
