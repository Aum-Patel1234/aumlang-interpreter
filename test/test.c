#include <stdio.h>
#include <assert.h>
#include <string.h>
#include "utils.h"
#include "vector.h"

/* ---------- Test helpers ---------- */

#define TEST_START(name) printf(BLUE("[TEST] %s ... "), name)
#define TEST_PASS() printf(GREEN("OK\n"))

/* ---------- Tests ---------- */

void test_int_vector(void) {
  TEST_START("int vector");

  vector v;
  vector_init(&v, sizeof(int));

  for (int i = 0; i < 100; i++) {
    vector_add(&v, &i);
  }

  assert(v.len == 100);

  for (int i = 0; i < 100; i++) {
    int* val = (int*)vector_get(&v, i);
    assert(val != NULL);
    assert(*val == i);
  }

  free_vector(&v);
  TEST_PASS();
}

void test_double_vector(void) {
  TEST_START("double vector");

  vector v;
  vector_init(&v, sizeof(double));

  for (int i = 0; i < 50; i++) {
    double x = i * 0.5;
    vector_add(&v, &x);
  }

  for (int i = 0; i < 50; i++) {
    double* val = (double*)vector_get(&v, i);
    assert(val != NULL);
    assert(*val == i * 0.5);
  }

  free_vector(&v);
  TEST_PASS();
}

typedef struct {
  int id;
  char name[16];
} Person;

void test_struct_vector(void) {
  TEST_START("struct vector");

  vector v;
  vector_init(&v, sizeof(Person));

  Person p1 = {1, "Alice"};
  Person p2 = {2, "Bob"};

  vector_add(&v, &p1);
  vector_add(&v, &p2);

  Person* r1 = (Person*)vector_get(&v, 0);
  Person* r2 = (Person*)vector_get(&v, 1);

  assert(r1->id == 1);
  assert(strcmp(r1->name, "Alice") == 0);
  assert(r2->id == 2);
  assert(strcmp(r2->name, "Bob") == 0);

  free_vector(&v);
  TEST_PASS();
}

void test_pointer_vector(void) {
  TEST_START("pointer vector");

  vector v;
  vector_init(&v, sizeof(char*));

  char* a = "hello";
  char* b = "world";

  vector_add(&v, &a);
  vector_add(&v, &b);

  char** r1 = (char**)vector_get(&v, 0);
  char** r2 = (char**)vector_get(&v, 1);

  assert(strcmp(*r1, "hello") == 0);
  assert(strcmp(*r2, "world") == 0);

  free_vector(&v);
  TEST_PASS();
}

void test_out_of_bounds(void) {
  TEST_START("out of bounds access");

  vector v;
  vector_init(&v, sizeof(int));

  int x = 10;
  vector_add(&v, &x);

  assert(vector_get(&v, 1) == NULL);
  assert(vector_get(&v, 100) == NULL);

  free_vector(&v);
  TEST_PASS();
}

void test_vector_reverse_int(void) {
  TEST_START("vector reverse (int)");

  vector v;
  vector_init(&v, sizeof(int));

  for (int i = 0; i < 10; i++) {
    vector_add(&v, &i); // [0 1 2 3 4 5 6 7 8 9]
  }

  vector_reverse(&v); // [9 8 7 6 5 4 3 2 1 0]

  for (int i = 0; i < 10; i++) {
    int* val = (int*)vector_get(&v, i);
    assert(val != NULL);
    assert(*val == 9 - i);
  }

  free_vector(&v);
  TEST_PASS();
}

void test_vector_reverse_struct(void) {
  TEST_START("vector reverse (struct)");

  vector v;
  vector_init(&v, sizeof(Person));

  Person p1 = {1, "Alice"};
  Person p2 = {2, "Bob"};
  Person p3 = {3, "Charlie"};

  vector_add(&v, &p1);
  vector_add(&v, &p2);
  vector_add(&v, &p3);

  vector_reverse(&v);

  Person* r0 = (Person*)vector_get(&v, 0);
  Person* r1 = (Person*)vector_get(&v, 1);
  Person* r2 = (Person*)vector_get(&v, 2);

  assert(r0->id == 3);
  assert(strcmp(r0->name, "Charlie") == 0);

  assert(r1->id == 2);
  assert(strcmp(r1->name, "Bob") == 0);

  assert(r2->id == 1);
  assert(strcmp(r2->name, "Alice") == 0);

  free_vector(&v);
  TEST_PASS();
}

/* ---------- Main ---------- */

int main(void) {
  printf("Running vector tests...\n\n");

  test_int_vector();
  test_double_vector();
  test_struct_vector();
  test_pointer_vector();
  test_out_of_bounds();

  test_vector_reverse_int();
  test_vector_reverse_struct();

  printf("\n%s\n", GREEN("All tests passed"));
  return 0;
}
