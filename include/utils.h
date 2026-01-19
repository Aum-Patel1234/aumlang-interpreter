#pragma once
#include "token.h"
#include <stdint.h>
#include <string.h>

void str_cpy(char* str, const char* start, size_t len) {
  strncpy(str, start, len);
  str[len] = '\0';
}

uint8_t isdouble(const char* str) { return strchr(str, '.') ? 1 : 0; }
