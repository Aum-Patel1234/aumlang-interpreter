#include <glib.h>
#include <stdio.h>
#include <stdlib.h>
#include "../include/lexer.h"
#include "../include/map.h"
#include "../include/cli.h"

int main(int argv, char* argc[]) {
  if (argv > 2) {
    printf("It only accepts 2 args, 1st the execuatble and 2nd the file name.\n");
    return 1;
  }

  GHashTable* token_map = init_hash_map();
  if (argv == 1) {
    run_cli(token_map);

    print_hash_map(token_map);
    free_hash_map(token_map);
    return 0;
  }

  const char* file_name = argc[1];
  FILE* file = fopen(file_name, "r");
  if (!file) {
    perror("can't open the file\n");
    return 1;
  }

  // get the size of file
  fseek(file, 0, SEEK_END);
  unsigned long size = (unsigned long)ftell(file);
  rewind(file);

  char* file_content = malloc(size + 1); // for '\0'
  if (!file_content) {
    perror("malloc");
    fclose(file);
    return 1;
  }

  fread(file_content, 1, size, file);
  file_content[size] = '\0';

  run_program(file_content, size + 1, token_map);
  print_hash_map(token_map);

  free_hash_map(token_map);
  free(file_content);
  fclose(file);

  return 0;
}
