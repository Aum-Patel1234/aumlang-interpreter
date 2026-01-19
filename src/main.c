#include <glib.h>
#include <stdio.h>
#include <stdlib.h>
#include "../include/lexer.h"

int main(int argv, char* argc[]) {
  if (argv > 2) {
    printf("It only accepts 2 args, 1st the execuatble and 2nd the file name.\n");
    return 1;
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

  run_program(file_content, size + 1);

  free(file_content);
  fclose(file);

  // printf("Welcone to Aumlang CLI\n");
  //
  // char input[256];
  // while (1) {
  //     printf("aum > ");
  //
  //     if (!fgets(input, sizeof(input), stdin))
  //         break;
  //
  //     size_t len = strlen(input);
  //     // check for enter key
  //     if (len > 0 && input[len - 1] == '\n')
  //         input[len - 1] = '\0';
  //
  //     if (strcmp(input, "exit") == 0)
  //         break;
  //
  //     printf("You typed: %s\n", input);
  // }
  //
  return 0;
}
