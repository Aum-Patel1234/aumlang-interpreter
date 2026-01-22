#include "../include/lexer.h"
#include <../include/eval_expr.h>
#include "glib.h"
#include "token.h"
#include "vector.h"
#include <ctype.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
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

void process_line(const char* line_start, const char* line_end, GHashTable* token_map) {
  uint8_t num_tokens = get_num_tokens(line_start, line_end);
  fwrite(line_start, sizeof(char), (size_t)(line_end - line_start), stdout);
  printf("\t\t\t num_tokens - %d\t\t\t", num_tokens);

  // NOTE: for now I am just doing for atmost 3 and not AST(Abstract Syntax Tree)
  switch (num_tokens) {
    case 1:
      break;
    case 2: {
      // TODO:
      // print a
      const char* next = strchr(line_start, ' ');
      // keyword token
      Token keyword_token = get_keyword_token(line_start, (size_t)(next - line_start));

      // value token
      const char* value_start = next + 1;
      size_t value_len = (size_t)(line_end - value_start);
      if (value_len > 0 && value_start[value_len - 1] == '\n')
        value_len--;

      Token value_token;
      if (isdigit(value_start[0]))
        value_token = get_literal_token(value_start, value_len);
      else
        value_token = get_identifier_token(value_start, value_len);

      printf("\n");
      print_token(&keyword_token);
      print_token(&value_token);
      break;
    }
    default: {
      // IMPORTANT: eval expression using Prat Parsing technique
      // expecting: variable = 1+2*...
      const char* next = strchr(line_start, '=');
      if (!next)
        return;

      // LHS
      const char* lhs_end = next;
      while (lhs_end > line_start && lhs_end[-1] == ' ')
        lhs_end--;

      size_t lhs_size = (size_t)(lhs_end - line_start); // - 1 for the space before =

      char lhs[lhs_size + 1];
      memcmp(lhs, line_start, lhs_size);
      lhs[lhs_size] = '\0';

      const char* rhs_start = next + 1;
      while (*rhs_start == ' ')
        rhs_start++;

      // RHS
      size_t rhs_size = (size_t)(line_end - rhs_start);
      char rhs[rhs_size + 1];
      memcmp(rhs, rhs_start, rhs_size);
      rhs[rhs_size] = '\0';

      double val = calculate(rhs);
    }
  }

  printf("\n\n\n");
}

void run_program(const char* code, unsigned long size) {
  const char* temp = code;
  GHashTable* token_map = g_hash_table_new(g_direct_hash, g_direct_equal);

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

    process_line(p, line, token_map);

    temp = line + 1;
  }

  GHashTableIter iter;
  gpointer key, val;
  g_hash_table_iter_init(&iter, token_map);
  while (g_hash_table_iter_next(&iter, key, val)) {
    free_token(key);
    free_token(val);
  }
}
