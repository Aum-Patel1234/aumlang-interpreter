#pragma once
#include <stddef.h>

typedef enum {
  KEYWORD_INVALID = 0,

  KEYWORD_PRINT,
  KEYWORD_FUNC,
  KEYWORD_RETURN,
  KEYWORD_GET_REF_COUNT,

  KEYWORD_IF,
  KEYWORD_ELSE,
  KEYWORD_WHILE,
  KEYWORD_FOR,

  KEYWORD_COUNT
} Keyword;

static const char* KEYWORD_STRINGS[KEYWORD_COUNT] =
    {"INVALID", "print", "func", "return", "getRefCount", "if", "else", "while", "for"};
static const size_t KEYWORD_STRINGS_SIZE = KEYWORD_COUNT;
