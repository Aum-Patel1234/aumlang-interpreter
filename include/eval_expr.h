#include "glib.h"
#include "token.h"
#include "vector.h"
#include <stdint.h>
#include <stddef.h>

typedef struct pair {
  float lhs, rhs;
} float_pair;

Token calculate(char* s, const GHashTable* token_map);
Token prat_parser(const vector* v);
