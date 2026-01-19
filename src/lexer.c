#include "../include/lexer.h"
#include "token.h"
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

char* get_line(const char* code) { return strchr(code, NEW_LINE); }

uint8_t get_num_tokens(const char* line, const char* line_end) {
  uint8_t size = 0, in_token = 0;

  for (const char* p = line; p != line_end; ++p) {
    if (*p == SEPERATOR) {
      in_token = 0;
    } else if (!in_token) {
      in_token = 1;
      size++;
    }
  }

  return size;
}

void run_program(const char* code, unsigned long size) {
  const char* temp = code;

  while (*temp) {
    const char* line = get_line(temp);
    if (!line)
      break;

    /* skip leading spaces */
    const char* p = temp;
    while (*p == ' ')
      ++p;

    /* skip comments and empty lines */
    if (*p == COMMENT_TAG || *p == NEW_LINE) {
      temp = (*line == '\n') ? line + 1 : line;
      continue;
    }

    temp = p;
    uint8_t num_tokens = get_num_tokens(temp, line);
    fwrite(temp, sizeof(char), line - temp, stdout);
    printf("\t\t\t num_tokens - %d\t\t\t", num_tokens);

    // NOTE: for now I am just doing for atmost 3 and not AST(Abstract Syntax Tree)
    switch (num_tokens) {
      case 1:
        break;
      case 2: {
        // print a
        const char* next = strchr(temp, ' ');
        // keyword token
        Token keyword_token = get_keyword_token(temp, (next - temp));

        // value token
        const char* value_start = next + 1;
        size_t value_len = line - value_start;
        if (value_len > 0 && value_start[value_len - 1] == '\n')
          value_len--;
        Token value_token;
        if (isdigit(value_start[0])) {
          value_token = get_literal_token(value_start, value_len);
        } else {
          value_token = get_identifier_token(value_start, value_len);
        }

        printf("\n");
        print_token(&keyword_token);
        print_token(&value_token);
        break;
      }
      case 3:
        break;
    }

    printf("\n\n\n");

    temp = line + 1;
  }
}
