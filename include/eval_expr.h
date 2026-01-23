#include "glib.h"
#include "token.h"
#include "vector.h"
#include <stdint.h>
#include <stddef.h>

Token calculate(char* s, const GHashTable* token_map);
Token prat_parser(const vector* v);
