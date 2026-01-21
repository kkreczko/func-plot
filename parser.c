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
    if (list->name)
      printf("%s; ", list->name);
    if (list->token == TOK_Number)
      printf("VALUE -> %f;", list->value);
    printf("\n");
    list = list->next;
  }
}

// ??? scp-69420
int isReserved(char name) {
  switch (name) {
  case 's':
    return 1;
  case 'c':
    return 1;
  case 'p':
    return 1;
  default:
    return 0;
  }
}

void parseNumbers(Node *list) { 

  return; 
}

Node parseExpr(char *expr) {
  int i = 0;
  Node *result = (Node *)malloc(sizeof(Node));

  while (expr[i] != '\0') {
    Node *current = (Node *)malloc(sizeof(Node));

    if (isdigit(expr[i])) {
      char *newValue = (char *)malloc(sizeof(char));
      *newValue = expr[i];
      current->token = TOK_Number;
      current->value = atof(newValue);
      current->name = "NUMBER";
      addItem(result, current);
      i++;
      continue;
    }

    if (isalpha(expr[i]) && !isReserved(expr[i])) {
      current->token = TOK_Var;
      current->name = (char *)malloc(sizeof(expr[i]));
      memcpy(current->name, &expr[i], sizeof(expr[i]));
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
      break;
    case '/':
      current->token = TOK_Divide;
      current->name = "DIVIDE";
      break;
    case '*':
      current->token = TOK_Multiply;
      current->name = "MULTIPLY";
      break;
    case '(':
      current->token = TOK_ParenOpen;
      current->name = "PARENOPEN";
      break;
    case ')':
      current->token = TOK_ParenClose;
      current->name = "PARENCLOSE";
      break;
    case '^':
      current->token = TOK_Power;
      current->name = "POWER";
      break;
    case 's':
      current->token = TOK_Sin;
      current->name = "SINUS";
      break;
    case 'c':
      current->token = TOK_Sin;
      current->name = "COSINUS";
      break;
    case 'p':
      current->token = TOK_Pi;
      current->name = "PI";
      break;
    default:
      current->token = TOK_Err;
      current->name = "ERR";
      break;
    }

    addItem(result, current);

    i++;
  }

  parseNumbers(result);
  return *result;
}
