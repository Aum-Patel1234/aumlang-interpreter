#include "../include/vector.h"
#include "../include/eval_expr.h"
#include "glib.h"
#include "token.h"
#include "utils.h"
#include <ctype.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

Token calculate(char* s, GHashTable* token_map) {
  char* temp = s;
  vector v;
  vector_init(&v, sizeof(Token));

  while (*temp != '\0') {
    // skip spaces
    if (*temp == ' ') {
      temp++;
      continue;
    }

    // number literal (int / double)
    if (isdigit(*temp)) {
      const char* start = temp;
      while (isdigit(*temp))
        temp++;

      size_t size = (size_t)(temp - start);
      Token token = get_literal_token(start, size);
      vector_add(&v, &token);
      continue;
    }

    // operator or parenthesis
    if (is_operator(*temp)) {
      Token token = get_operator_token(*temp);
      vector_add(&v, &token);
      temp++;
      continue;
    }

    // identifier (variable)
    if (isalpha(*temp) || *temp == '_') {
      const char* start = temp;
      while (isalnum(*temp) || *temp == '_')
        temp++;

      size_t size = (size_t)(temp - start);
      Token lookup = get_identifier_token(start, size);
      Token* val = g_hash_table_lookup(token_map, &lookup);
      free_static_token(&lookup);

      if (!val) {
        free_vector_custom(&v, (element_free_fn)free_static_token);
        perror("Variable name not defined");
        return (Token){0};
      }

      Token token = {0};
      token_deep_copy(&token, val);
      vector_add(&v, &token);
      continue;
    }

    perror("Unknown character in expression");
    free_vector_custom(&v, (element_free_fn)free_static_token);
    return (Token){0};
  }

  // print_token_vector(&v);
  vector_reverse(&v);

  Token ans = prat_parser(&v, 0);
  free_vector_custom(&v, (element_free_fn)free_static_token);

  return ans;
}

static double token_to_double(const Token* t) {
  if (t->literal_kind == LITERAL_INT)
    return (double)t->value.int_value;

  return t->value.double_value;
}

Token operation(Token* lhs, char op, Token* rhs) {
  Token result = {0};
  result.type = LITERAL;
  result.literal_kind = LITERAL_DOUBLE;

  double a = token_to_double(lhs);
  double b = token_to_double(rhs);

  switch (op) {
    case '+':
      result.value.double_value = a + b;
      break;
    case '-':
      result.value.double_value = a - b;
      break;
    case '*':
      result.value.double_value = a * b;
      break;
    case '/':
      result.value.double_value = a / b;
      break;
    default:
      perror("invalid operator");
      return (Token){0};
  }

  return result;
}

Token prat_parser(vector* v, float min_bp) {
  Token lhs = (Token){0};
  if (is_empty(v))
    return lhs;

  Token* last_elem = (Token*)back(v);
  if (!last_elem) {
    perror("Malformed expression: lhs is NULL");
    return lhs;
  }
  // handle '('
  if (last_elem->type == OPERATOR && last_elem->value.ch == '(') {
    vector_pop_back_token(v);
    lhs = prat_parser(v, 0); // IMPORTANT: ( should lowest binding power to eval inner expression

    Token* end = (Token*)back(v);
    if (end->type != OPERATOR || end->value.ch != ')')
      perror("'(' bracket did not find corresponding ')' bracket.");
    else
      vector_pop_back_token(v); // pop '('
  }
  // handle literal
  else if (last_elem->type == LITERAL) {
    token_deep_copy(&lhs, last_elem);
    vector_pop_back_token(v);
    // **normalize to double**
    if (lhs.literal_kind == LITERAL_INT) {
      lhs.value.double_value = (double)lhs.value.int_value;
      lhs.literal_kind = LITERAL_DOUBLE;
    }
  }
  // handle unary minus
  else if (last_elem->type == OPERATOR && last_elem->value.ch == '-') {
    vector_pop_back_token(v);
    Token rhs = prat_parser(v, 100); // high min_bp for unary
    lhs.type = LITERAL;
    lhs.literal_kind = LITERAL_DOUBLE;
    lhs.value.double_value = -token_to_double(&rhs);
    free_static_token(&rhs);
  } else {
    perror("Unexpected token as lhs");
    return (Token){0};
  }
  // printf("\n");
  // print_token(&lhs);

  while (!is_empty(v)) {
    Token* op = (Token*)back(v);
    if (!op || op->type != OPERATOR || op->value.ch == ')')
      break;

    float_pair bp = infix_binding_power(op->value.ch);
    if (bp.lhs < min_bp)
      break;

    char ch = op->value.ch;
    // IMPORTANT: as i am freeing the operator memmory is can see that memmory again
    // Debugging this error for 15 mins
    vector_pop_back_token(v); // consume operator

    Token rhs = prat_parser(v, bp.rhs);

    Token tmp = operation(&lhs, ch, &rhs);
    free_static_token(&lhs);
    free_static_token(&rhs);
    lhs = tmp;
  }

  return lhs;
}

float_pair infix_binding_power(char ch) {
  // if (ch == '=')
  //   return (float_pair){.lhs = (float)0.2, .rhs = (float)0.1};
  if (ch == '+' || ch == '-')
    return (float_pair){.lhs = (float)1.0, .rhs = (float)1.1};
  // if(ch == '*' || ch == '/')
  return (float_pair){.lhs = (float)2.0, .rhs = (float)2.1};
}
