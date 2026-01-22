
typedef struct Vector{
    void* val;
    size_t len, capacity, elem_size;
}vector;

void init(vector* v);
void re_init(vector* v);
void add(vector* v, int val);
void free_vector(vector* v);