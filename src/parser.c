#include "parser.h"
#include "reorder.h"

#include <ctype.h>
#include <math.h>
#include <stddef.h>
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

int isReserved(char name) {
  switch (name) {
  case 's':
    return 1;
  case 'c':
    return 1;
  case 'p':
    return 1;
  case 'e':
    return 1;
  default:
    return 0;
  }
}

void dump(Node *list) {
  while (list) {
    if (list->token == TOK_Number || list->token == TOK_Pi ||
        list->token == TOK_Euler)
      printf("%s; VALUE -> %f\n", list->name, list->value);
    else if (list->name)
      printf("%s\n", list->name);
    list = list->next;
  }
}

double calculateValue(double *nodeValues, int exponent) {
  double result = 0;

  for (int i = exponent; i >= 0; i--) {
    result += nodeValues[exponent - i] * pow(10, i);
  }

  return result;
}

void combineNumbers(Node *list) {
  double values_buff[NODE_BUFF];
  int exponent = -1;

  while (list) {
    if (list->token == TOK_Number) {
      exponent++;
      values_buff[exponent] = list->value;
      list->value = calculateValue(values_buff, exponent);
    } else {
      memset(values_buff, 0, sizeof(values_buff));
      exponent = -1;
    }
    list = list->next;
  }
}

void removeRedundantNumbers(Node *list) {
  Node *head;

  while (list->next) {
    if (list->next->token == TOK_Number && list->token != TOK_Number) {
      head = list;
    } else if (list->next->token != TOK_Number && list->token == TOK_Number) {
      head->next = list;
    }
    list = list->next;
  }
}

Node parseExpr(char *expr) {
  int i = 0;
  Node *result = malloc(sizeof(Node));

  result->token = TOK_Start;
  result->name = "START";

  while (expr[i] != '\0') {
    Node *current = malloc(sizeof(Node));

    if (isdigit(expr[i])) {
      char *newValue = malloc(sizeof(char));
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
      current->name = malloc(sizeof(expr[i]));
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
      current->value = 3.1415926535;
      break;
    case 'e':
      current->token = TOK_Euler;
      current->name = "EULER";
      current->value = 2.71828;
      break;
    default:
      current->token = TOK_Err;
      current->name = "ERR";
      break;
    }

    addItem(result, current);

    i++;
  }

  Node *end = malloc(sizeof(Node));
  end->token = TOK_End;
  end->name = "FIN";
  addItem(result, end);

  combineNumbers(result);
  removeRedundantNumbers(result);
  reorderToPolishNotation(result);
  return *result;
}
