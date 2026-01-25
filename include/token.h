#pragma once

#include "vector.h"
#include <stddef.h>
#include <ctype.h>

typedef enum Tokens {
  INVALID_TOKEN = 0,
  KEYWORD,
  IDENTIFIER,
  LITERAL,
  OPERATOR, // only support +,-,/,*,(,)
  PUNCTUATOR,
} token_type;

typedef enum { LITERAL_INT, LITERAL_DOUBLE, LITERAL_STRING, LITERAL_CHAR } literal_type;

typedef union token_value { // good use of union
  int int_value;
  double double_value;
  char* string_literal;
  char ch;
} token_value;

typedef struct token_obj {
  token_type type;
  char* variable_name;
  literal_type literal_kind; // only meaningful if type == LITERAL
  token_value value;
} Token;

const char* token_type_to_string(token_type type);
void print_token(const Token* token);
void free_static_token(Token* token);
void free_dynamic_token(Token* token);
void token_deep_copy(Token* dst, const Token* src);
void print_token_vector(const vector* v);

Token get_literal_token(const char* str, size_t size);
Token get_keyword_token(const char* str, size_t size);
Token get_identifier_token(const char* str, size_t size);
Token get_operator_token(const char ch);
void set_token_value(
    Token* token, token_type type, literal_type literal, token_value value, char* variable_name);
