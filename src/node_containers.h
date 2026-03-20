#ifndef NODE_CONTAINERS_H
#define NODE_CONTAINERS_H

#include "parser.h"

#define CONTAINER_SIZE 256

typedef struct NodeContainer {
  Node items[CONTAINER_SIZE];
  int stackPtr;
} NodeContainer;

void dumpContainer(NodeContainer *list);
void push(NodeContainer *list, Node *item);
Node *pop(NodeContainer *list);

#endif // !NODE_CONTAINERS_H
