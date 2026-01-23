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

void free_vector(vector* v) {
  free(v->val);
  v->val = NULL;
  v->len = v->capacity = v->elem_size = 0;
}

void* vector_get(const vector* v, size_t index) {
  return index >= v->len ? NULL : (char*)v->val + (v->elem_size * index);
}
