#include <stdint.h>
#define NEW_LINE '\n'
#define COMMENT_TAG '#'
#define SEPERATOR ' '
#include <stdio.h>
#include <string.h>

uint8_t get_num_tokens(const char* line, const char* line_end);
void run_program(const char* code, unsigned long size);
char* get_line(const char* code);
