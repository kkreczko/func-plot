#include "node_containers.h"

#include "stdlib.h"

void dumpContainer(NodeContainer *container) {
  for (int i = 0; i < container->stackPtr; i++) {
    Node temp_copy = container->items[i];
    temp_copy.next = NULL;
    dump(&temp_copy);
  }
}

void push(NodeContainer *container, Node *item) {
  container->items[container->stackPtr++] = *item;
}

Node *pop(NodeContainer *container) {
  return &container->items[--container->stackPtr];
}
