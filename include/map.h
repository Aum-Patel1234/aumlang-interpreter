#include "glib.h"
#include "glibconfig.h"
#include "token.h"
#include <string.h>

gboolean token_equal(gconstpointer a, gconstpointer b);
guint token_hash(gconstpointer key);
GHashTable* init_hash_map(void);
void free_hash_map(GHashTable* token_map);
void insert_hash_map(GHashTable* token_map, Token* key, Token* val);
