#ifndef NODE_CONTAINERS_H
#define NODE_CONTAINERS_H

#include "parser.h"

typedef struct NodeContainer {
  Node *current;
  struct NodeContainer *next;
} NodeContainer;

void dump(NodeContainer *list);
void push(NodeContainer *list);
Node pop(NodeContainer *list);
void append(NodeContainer *list);

#endif // !NODE_CONTAINERS_H
