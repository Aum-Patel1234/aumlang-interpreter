#include <stdio.h>
#include <stdlib.h>
#include<stddef.h>

void init(vector* v, size_t elem_size){
    v->capacity = 10;
    v->val = malloc(v->capacity * sizeof(elem_size));
    if(v->val == NULL){
        // TODO: handle error
    }
    v->len = 0;
    v->elem_size = elem_size;
}
void re_init(vector* v){
    v->capacity *= 2;
    v->val = realloc(v->val, v->capacity * sizeof(v->elem_size));
    if(v->val == NULL){
        // TODO: handle err
    }
}

void add(vector* v, void* val){
    if(v->len == v->capacity){
        re_init(v);
    }
    v->val[v->len] = val;
    v->len++;
}

void free_vector(vector* v){
    free(v->val);
}

// int main(){
//     vector v;
//     init(&v);

//     for (int i = 0; i < 13; i++) {
//         add(&v, 1);
//     }

//     printf("%zu\t%zu\n", v.len, v.capacity);
//     for (size_t i = 0; i < v.len; i++) {
//         printf("%d ", v.val[i]);
//     }

//     free(v.val);
//     return 0;
// }