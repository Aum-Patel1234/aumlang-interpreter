#include "../include/cli.h"
#include "../include/lexer.h"

void run_cli(GHashTable* token_map) {
  printf("Welcone to Aumlang CLI\n");

  char input[256];
  while (1) {
    printf("aum > ");

    if (!fgets(input, sizeof(input), stdin))
      break;

    size_t len = strlen(input);
    // check for enter key
    if (len > 0 && input[len - 1] == '\n') {
      input[len - 1] = '\0';
      len--;
    }

    if (strcmp(input, "exit") == 0)
      break;

    char* end = input + len;
    process_line(input, end, token_map);
  }
}
