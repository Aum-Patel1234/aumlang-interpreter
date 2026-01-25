#pragma once

#include "glib.h"
#include "eval_expr.h"
#include "map.h"
#include "token.h"
#include "glib.h"
#include <assert.h>
#include <ctype.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#define NEW_LINE '\n'
#define COMMENT_TAG '#'
#define SEPERATOR ' '

uint8_t get_num_tokens(const char* line, const char* line_end);
void process_line(const char* line_start, const char* line_end, GHashTable* token_map);
void run_program(const char* code, unsigned long size, GHashTable* token_map);
char* get_line(const char* code);
