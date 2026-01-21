#include "token.h"
#include <stdint.h>
#define NEW_LINE '\n'
#define COMMENT_TAG '#'
#define SEPERATOR ' '
#include <stdio.h>
#include <string.h>

typedef struct {
  Token* data;
  size_t len;
  size_t elem_size;
} Token_Array;

uint8_t get_num_tokens(const char* line, const char* line_end);
Token_Array get_tokens(const char* temp, const char* line);
void run_program(const char* code, unsigned long size);
char* get_line(const char* code);
