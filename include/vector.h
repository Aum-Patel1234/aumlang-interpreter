#pragma once

#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <string.h>
#define INITIAL_CAPACITY 8

typedef struct Vector {
  void* val;
  size_t len, capacity, elem_size;
} vector;

void vector_init(vector* v, size_t elem_size);
void vector_re_init(vector* v);
void vector_add(vector* v, const void* val);
void free_vector(vector* v);
void* vector_get(const vector* v, size_t index);
