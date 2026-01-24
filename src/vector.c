#include "token.h"
#include <../include/vector.h>

void vector_init(vector* v, size_t elem_size) {
  v->capacity = INITIAL_CAPACITY;
  v->len = 0;
  v->elem_size = elem_size;
  v->val = malloc(v->capacity * elem_size);
  if (!v->val) {
    perror("malloc failed");
    exit(EXIT_FAILURE);
  }
}

void vector_init_with_capacity(vector* v, size_t elem_size, size_t initial_capactiy) {
  v->capacity = initial_capactiy;
  v->len = 0;
  v->elem_size = elem_size;
  v->val = malloc(v->capacity * elem_size);
  if (!v->val) {
    perror("malloc failed");
    exit(EXIT_FAILURE);
  }
}

void vector_re_init(vector* v) {
  v->capacity *= 2;
  v->val = realloc(v->val, v->capacity * v->elem_size);
  if (!v->val) {
    perror("realloc failed");
    exit(EXIT_FAILURE);
  }
}

void vector_add(vector* v, const void* val) {
  if (v->len == v->capacity)
    vector_re_init(v);
  // IMPORTANT:
  // (char*)v->data              // treat memory as bytes
  // + len * elem_size           // jump to next slot
  // memcpy(..., elem, size)     // copy actual value
  void* dest = (char*)v->val + (v->len * v->elem_size);
  memcpy(dest, val, v->elem_size);
  v->len++;
}

void vector_reverse(vector* v) {
  size_t elem_size = v->elem_size, len = v->len;
  char* data = (char*)v->val;

  void* temp = malloc(elem_size);
  if (!temp) {
    perror("malloc failed");
    return;
  }

  for (size_t i = 0; i < len / 2; ++i) {
    char* left = data + i * elem_size;
    char* right = data + (len - i - 1) * elem_size;

    memcpy(temp, left, elem_size);
    memcpy(left, right, elem_size);
    memcpy(right, temp, elem_size);
  }

  free(temp);
}

// NOTE: this func is not general only for Token*
void vector_pop_back_token(vector* v) {
  Token* last_elem = (Token*)back(v);
  if (last_elem) {
    free_static_token(last_elem);
    v->len--;
  }
}

uint8_t is_empty(const vector* v) { return v->len == 0; }

void* back(const vector* v) {
  if (v->len == 0)
    return NULL;
  char* data = (char*)v->val;
  return data + (v->len - 1) * v->elem_size;
}

// NOTE: element_free_fn should take one argument and free the indivisual elements
void free_vector_custom(vector* v, element_free_fn free_elem) {
  if (!v || !v->val)
    return;

  char* data = (char*)v->val;
  if (free_elem) {
    for (size_t i = 0; i < v->len; ++i) {
      void* elem = data + i * v->elem_size;
      free_elem(elem); // call user-provided free function
    }
  }

  free(v->val);
  v->val = NULL;
  v->len = v->capacity = v->elem_size = 0;
}

// void free_vector(vector* v) {
//   if (!v || !v->val)
//     return;
//   char* data = (char*)v->val;
//   for (size_t i = 0; i < v->len; ++i) {
//     Token* t = (Token*)(data + i * v->elem_size);
//     free_static_token(t);
//   }
//   free(v->val);
//   v->val = NULL;
//   v->len = v->capacity = v->elem_size = 0;
// }

void* vector_get(const vector* v, size_t index) {
  return index >= v->len ? NULL : (char*)v->val + (v->elem_size * index);
}
