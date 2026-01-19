#include <stddef.h>

static const char* KEYWORD_STRINGS[] = {"print", "func", "return", "getRefCount"};
static const size_t KEYWORD_STRINGS_SIZE = sizeof(KEYWORD_STRINGS) / sizeof(KEYWORD_STRINGS[0]);

typedef enum {
  KEYWORD_INVALID = 0,

  KEYWORD_PRINT,
  KEYWORD_FUNC,
  KEYWORD_RETURN,
  KEYWORD_GET_REF_COUNT,

  KEYWORD_COUNT
} Keyword;
