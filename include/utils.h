#pragma once
#include "token.h"
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define RED(text) "\033[31m" text "\033[0m"
#define GREEN(text) "\033[32m" text "\033[0m"
#define YELLOW(text) "\033[33m" text "\033[0m"
#define BLUE(text) "\033[34m" text "\033[0m"
#define MAGENTA(text) "\033[35m" text "\033[0m"
#define CYAN(text) "\033[36m" text "\033[0m"

// strcpy but adds \0 at the index len
void str_cpy(char* str, const char* start, size_t len);
uint8_t isdouble(const char* str);
uint8_t is_operator(char ch);
char* evaluate_between_parenthesis(const char* begin, const char* end);