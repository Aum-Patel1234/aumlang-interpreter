#pragma once

#include "glib.h"
#include "glib.h"
#include "../include/token.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

gboolean token_equal(gconstpointer a, gconstpointer b);
guint token_hash(gconstpointer key);
GHashTable* init_hash_map(void);
void free_hash_map(GHashTable* token_map);
void insert_hash_map(GHashTable* token_map, Token* key, Token* val);
void print_hash_map(const GHashTable* token_map);
