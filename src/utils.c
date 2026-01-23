#include "../include/utils.h"
#include <stdint.h>

void str_cpy(char* str, const char* start, size_t len) {
  strncpy(str, start, len);
  str[len] = '\0';
}

uint8_t isdouble(const char* str) { return strchr(str, '.') ? 1 : 0; }

uint8_t is_operator(char ch) {
  return (ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '(' || ch == ')');
}
