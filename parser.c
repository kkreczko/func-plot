#include "parser.h"
#include <ctype.h>
#include <stdlib.h>

void addItem(NodeList *list, Node *item) {
  struct NodeList *new = (struct NodeList *)malloc(sizeof(NodeList));
  new->curr = *item;
  new->next = NULL;
  list->next = new;
}

NodeList parseExpr(char expr[]) {
  int i = 0;
  NodeList result;

  while (expr[i] != '\0') {
    Node current;

    if (isdigit(expr[i])) {
      current.token = TOK_Number;
      current.value = atof(&expr[i]);
      addItem(&result, &current);
      i++;
      continue;
    }

    switch (expr[i]) {
    case '+':
      current.token = TOK_Plus;
      break;
    case '-':
      current.token = TOK_Minus;
      break;
    case ' ':
      break;
    default:
      current.token = TOK_Err;
      break;
    }

    addItem(&result, &current);
    i++;
  }

  return result;
}
