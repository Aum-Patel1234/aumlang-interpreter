#include "../include/vector.h"
#include "../include/eval_expr.h"
#include "glib.h"
#include "token.h"
#include "utils.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>

Token calculate(char* s, const GHashTable* token_map) {
  char* temp = s;

  vector v;
  vector_init(&v, sizeof(Token));
  while (*temp != '\0') {
    // leading spaces
    while (*temp == ' ')
      temp++;
    if (*temp == '\0')
      break;

    const char* start = temp;
    /* move until space or end */
    while (*temp != ' ' && *temp != '\0')
      temp++;

    const size_t size = (size_t)(temp - start);
    char str[size + 1];
    str_cpy(str, start, size);

    Token token;
    if (size == 1 && is_operator(str[0]))
      token = get_operator_token(str[0]);
    else if (isdigit(str[0]))
      token = get_literal_token(str, size);
    else {
      // its a variable
      Token lookup = get_identifier_token(str, size);
      Token* val = g_hash_table_lookup(token_map, &lookup);
      free_static_token(&lookup);
      if (!val) {
        // handle exit if needed cause we have to free the token_map and the file_ptr
        free_vector_custom(&v, (element_free_fn)free_static_token);
        perror("Variable name  not defined\n");
        return (Token){0};
      }

      token_deep_copy(&token, val);
      // token = *val;
    }

    vector_add(&v, &token);
  }

  print_token_vector(&v);
  // easy to pop_back
  vector_reverse(&v);

  Token ans = prat_parser(&v);
  free_vector_custom(&v, (element_free_fn)free_static_token);

  return ans;
}

Token prat_parser(const vector* v) {
  Token t = {0};
  return t;
}

float_pair infix_binding_power(char ch) {
  // if (ch == '=')
  //   return (float_pair){.lhs = (float)0.2, .rhs = (float)0.1};
  if (ch == '+' || ch == '-')
    return (float_pair){.lhs = (float)1.0, .rhs = (float)1.1};
  // if(ch == '*' || ch == '/')
  return (float_pair){.lhs = (float)2.0, .rhs = (float)2.1};
}
