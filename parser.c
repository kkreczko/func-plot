#include "parser.h"

#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void addItem(Node *list, Node *item) {
  while (list->next) {
    list = list->next;
  }

  list->next = item;
  item->next = NULL;
}

void dump(Node *list) {
  while (list) {
    if (list->name && list->token != TOK_WhiteSpace)
      printf("%s\n", list->name);
    list = list->next;
  }
}

Node parseExpr(char *expr) {
  int i = 0;
  Node *result = (Node *)malloc(sizeof(Node));

  while (expr[i] != '\0') {
    Node *current = (Node *)malloc(sizeof(Node));

    if (isdigit(expr[i])) {
      current->token = TOK_Number;
      current->value = atof(expr + i);
      current->name = "NUMBER";
      addItem(result, current);
      i++;
      continue;
    }

    if (isalpha(expr[i])) {
      current->token = TOK_Var;
      current->name = (char *)malloc(sizeof(expr[i]));
      strcpy(current->name, expr + i);
      addItem(result, current);
      i++;
      continue;
    }

    switch (expr[i]) {
    case '+':
      current->token = TOK_Plus;
      current->name = "PLUS";
      break;
    case '-':
      current->token = TOK_Minus;
      current->name = "MINUS";
      break;
    case ' ':
      current->token = TOK_WhiteSpace;
      current->name = "WHITESPACE";
      break;
    default:
      current->token = TOK_Err;
      current->name = "ERR";
      break;
    }

    addItem(result, current);

    i++;
  }

  dump(result);
  return *result;
}
