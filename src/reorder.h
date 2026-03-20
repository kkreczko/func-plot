#ifndef REORDER_H
#define REORDER_H

#include "node_containers.h"

void reorderToPolishNotation(Node *result);
int getOperatorPriority(Node *item);
int isOperator(Node *item);
int isTrigFunc(Node *item);
int isParen(Node *item);
int isNumOrVar(Node *item);
int isEnd(Node *item);

#endif // !REORDER_H
