#pragma once

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <string.h>
#define INITIAL_CAPACITY 8

typedef struct Vector {
  void* val;
  size_t len, capacity, elem_size;
} vector;

typedef void (*element_free_fn)(void*);

void vector_init(vector* v, size_t elem_size);
void vector_init_with_capacity(vector* v, size_t elem_size, size_t initial_capactiy);
void vector_re_init(vector* v);
uint8_t is_empty(const vector* v);
void* back(const vector* v);
void vector_pop_back_token(vector* v);
void vector_add(vector* v, const void* val);
void vector_reverse(vector* v);
void free_vector_custom(vector* v, element_free_fn free_elem);
void* vector_get(const vector* v, size_t index);
