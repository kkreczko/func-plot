#include "parser.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>

void addItem(Node *list, Node *item) {
  while (list->next) {
    list = list->next;
  }

  list->next = item;
  item->next = NULL;
}

void dump(Node *list) {
  while (list) {
    if (list->name)
      printf("%s\n", list->name);
    list = list->next;
  }
}

Node parseExpr(char *expr) {
  int i = 0;
  Node *result = (Node *)malloc(sizeof(Node));

  while (*(expr + i) != '\0') {
    Node *current = (Node *)malloc(sizeof(Node));

    if (isdigit(*(expr + i))) {
      current->token = TOK_Number;
      current->value = atof(expr + i);
      current->name = "NUMBER";
      addItem(result, current);
      i++;
      continue;
    }

    switch (*(expr + i)) {
    case '+':
      current->token = TOK_Plus;
      current->name = "PLUS";
      addItem(result, current);
      break;
    case '-':
      current->token = TOK_Minus;
      current->name = "MINUS";
      addItem(result, current);
      break;
    case ' ':
      current->token = TOK_WhiteSpace;
      current->name = "WHITESPACE";
      addItem(result, current);
      break;
    default:
      current->token = TOK_Err;
      current->name = "ERR";
      addItem(result, current);
      break;
    }

    i++;
  }

  dump(result);
  return *result;
}
