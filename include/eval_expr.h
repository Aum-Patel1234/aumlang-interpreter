#include "glib.h"
#include "token.h"
#include "vector.h"
#include <stdint.h>
#include <stddef.h>

typedef struct pair {
  float lhs, rhs;
} float_pair;

Token calculate(char* s, GHashTable* token_map);
// returns result of the valid expression in double precision
Token prat_parser(vector* v, float min_bp);
float_pair infix_binding_power(char ch);
Token operation(Token* lhs, char op, Token* rhs);
