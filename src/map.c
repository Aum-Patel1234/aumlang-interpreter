#include "../include/map.h"

gboolean token_equal(gconstpointer a, gconstpointer b) {
  const Token *t1 = a, *t2 = b;

  if (t1->type != t2->type)
    return FALSE;

  if (t1->type == IDENTIFIER)
    return strcmp(t1->variable_name, t2->variable_name) == 0;

  return FALSE;
}

guint token_hash(gconstpointer key) {
  const Token* t = key;

  if (t->type == IDENTIFIER)
    return g_str_hash(t->variable_name);

  return 0;
}

GHashTable* init_hash_map(void) { return g_hash_table_new(token_hash, token_equal); }

void free_hash_map(GHashTable* token_map) {
  GHashTableIter iter;
  gpointer key = NULL, val = NULL;

  g_hash_table_iter_init(&iter, token_map);
  while (g_hash_table_iter_next(&iter, &key, &val)) {
    free_dynamic_token(key);
    free_dynamic_token(val);
  }

  g_hash_table_destroy(token_map);
}

void insert_hash_map(GHashTable* token_map, Token* key, Token* val) {
  Token* key_copy = malloc(sizeof(Token));
  Token* val_copy = malloc(sizeof(Token));

  if (!key_copy || !val_copy) {
    perror("malloc in hash_table failed");
    exit(EXIT_FAILURE);
  }

  token_deep_copy(key_copy, key);
  token_deep_copy(val_copy, val);
  g_hash_table_insert(token_map, key_copy, val_copy);
}

static void print_token_map_entry(gpointer key, gpointer value, gpointer user_data) {
  Token* val = (Token*)value;
  Token* k = (Token*)key;
  print_token(k);
  print_token(val);
  printf("\n");
}

void print_hash_map(const GHashTable* token_map) {
  if (!token_map) {
    printf("Hash table is empty.\n");
    return;
  }

  printf("=========== Token Map =========\n\n");
  g_hash_table_foreach((GHashTable*)token_map, print_token_map_entry, NULL);
  printf("===============================\n");
}
