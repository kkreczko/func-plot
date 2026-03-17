#ifndef REORDER_H
#define REORDER_H

#include "parser.h"

#define NODE_STACK_SIZE 256
#define NODE_QUEUE_SIZE 256

typedef struct NodeStack {
} NodeStack;

typedef struct NodeQueue {
} NodeQueue;

void reorderToPolishNotation(Node *result);
int getOperatorPriority(Node *item);
int isOperator(Node *item);
int isNumOrVar(Node *item);
int isEnd(Node *item);

#endif // !REORDER_H
