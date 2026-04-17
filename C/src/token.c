#include "../include/token.h"
#include "../include/utils.h"
#include "../include/keywords.h"

void set_token_value(
    Token* token, token_type type, literal_type literal, token_value value, char* variable_name) {
  *token = (Token){.type = type,
                   .literal_kind = literal,
                   .value = value,
                   .variable_name = (type == IDENTIFIER) ? variable_name : NULL};
}

Token get_literal_token(const char* str, size_t size) {
  Token token = {0};
  if (size == 0)
    return token;

  // check for character
  switch (str[0]) {
    case '\'': {
      // its a character
      if (size != 3 || str[2] != '\'') { // should be in form -> 'a'
        return token;
      }

      set_token_value(&token, LITERAL, LITERAL_CHAR, (token_value){.ch = str[1]}, NULL);
      break;
    }
    case '"': {
      // its a string
      const char* next = strchr(str + 1, '\"');
      if (!next)
        next = str + size; // pointer arithmetic (go to end)

      size_t len = (size_t)(next - (str + 1));

      char str_value[len + 1];
      str_cpy(str_value, str + 1, len);

      set_token_value(&token,
                      LITERAL,
                      LITERAL_STRING,
                      (token_value){.string_literal = str_value},
                      NULL);
      break;
    }
    default: {
      // assume its int or double
      if (!isdigit((unsigned char)str[0])) {
        fprintf(stderr, "Lexer error: invalid token starting with '%c'\n", str[0]);
        return token; // token is zeroed → invalid
      }

      if (isdouble(str)) {
        set_token_value(&token,
                        LITERAL,
                        LITERAL_DOUBLE,
                        (token_value){.double_value = atof(str)},
                        NULL);
      } else {
        set_token_value(&token, LITERAL, LITERAL_INT, (token_value){.int_value = atoi(str)}, NULL);
      }
    }
  }

  return token;
}

Token get_identifier_token(const char* str, size_t size) {
  Token token = {0};
  if (size == 0 || isdigit(str[0]))
    return token;

  // TODO: i have not written func to free it
  char* name = malloc(size + 1);
  if (!name)
    return token;
  str_cpy(name, str, size);

  set_token_value(&token, IDENTIFIER, 0, (token_value){0}, name);
  return token;
}

Token get_operator_token(const char ch) {
  Token token = {0};
  set_token_value(&token, OPERATOR, 0, (token_value){.ch = ch}, NULL);
  return token;
}

Token get_keyword_token(const char* str, size_t size) {
  Token token = {0};

  for (size_t i = 0; i < KEYWORD_STRINGS_SIZE; ++i) {
    if (strlen(KEYWORD_STRINGS[i]) == size && strncmp(str, KEYWORD_STRINGS[i], size)) {
      set_token_value(&token, KEYWORD, 0, (token_value){0}, NULL);
      break;
    }
  }
  token.type = INVALID_TOKEN;
  return token;
}

const char* token_type_to_string(token_type type) {
  switch (type) {
    case KEYWORD:
      return "KEYWORD";
    case IDENTIFIER:
      return "IDENTIFIER";
    case LITERAL:
      return "LITERAL";
    case OPERATOR:
      return "OPERATOR";
    case PUNCTUATOR:
      return "PUNCTUATOR";
    case INVALID_TOKEN:
    default:
      return "INVALID_TOKEN";
  }
}

void print_token(const Token* token) {
  if (!token || token->type == INVALID_TOKEN)
    return;
  printf("Type: %s ", token_type_to_string(token->type));

  switch (token->type) {
    case IDENTIFIER:
      printf("Value: %s", token->variable_name);
      break;

    case LITERAL:
      switch (token->literal_kind) {
        case LITERAL_INT:
          printf("Value: %d", token->value.int_value);
          break;

        case LITERAL_DOUBLE:
          printf("Value: %f", token->value.double_value);
          break;

        case LITERAL_CHAR:
          printf("Value: '%c'", token->value.ch);
          break;

        case LITERAL_STRING:
          printf("Value: \"%s\"", token->value.string_literal);
          break;
      }
      break;

    case KEYWORD:
      printf("Value: <keyword>");
      break;

    case OPERATOR:
      printf("Value: %c", token->value.ch);
      break;

    default:
      printf("Value: <invalid>");
      break;
  }

  printf("\n");
}

void free_static_token(Token* token) {
  if (!token)
    return;

  if (token->variable_name) {
    free(token->variable_name);
    token->variable_name = NULL;
  }

  if (token->type == LITERAL && token->literal_kind == LITERAL_STRING &&
      token->value.string_literal) {
    free(token->value.string_literal);
    token->value.string_literal = NULL;
  }
}

void free_dynamic_token(Token* token) {
  free_static_token(token);
  free(token);
}

void token_deep_copy(Token* dst, const Token* src) {
  memset(dst, 0, sizeof(*dst));

  dst->type = src->type;
  dst->literal_kind = src->literal_kind;

  // identifier
  if (src->type == IDENTIFIER && src->variable_name) {
    dst->variable_name = strdup(src->variable_name);
    if (!dst->variable_name) {
      perror("strdup failed");
      exit(EXIT_FAILURE);
    }
  } else {
    dst->variable_name = NULL;
  }

  dst->value = src->value;

  // deep copy string literal
  if (src->type == LITERAL && src->literal_kind == LITERAL_STRING && src->value.string_literal) {
    dst->value.string_literal = strdup(src->value.string_literal);
    if (!dst->value.string_literal) {
      perror("strdup failed");
      exit(EXIT_FAILURE);
    }
  }
}

void print_token_vector(const vector* v) {
  if (!v || v->len == 0) {
    printf("<empty vector>\n");
    return;
  }

  printf("Vector contents (size = %zu):\n", v->len);
  for (size_t i = 0; i < v->len; ++i) {
    Token* t = (Token*)vector_get(v, i);
    if (t) {
      printf("[%zu]: ", i);
      print_token(t);
    } else {
      printf("[%zu]: NULL\n", i);
    }
  }
}
