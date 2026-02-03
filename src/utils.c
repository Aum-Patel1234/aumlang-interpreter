#include "../include/utils.h"
#include <stdint.h>

void str_cpy(char* str, const char* start, size_t len) {
  strncpy(str, start, len);
  str[len] = '\0';
}

uint8_t isdouble(const char* str) { return strchr(str, '.') ? 1 : 0; }

uint8_t is_operator(char ch) {
  return (ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '(' || ch == ')');
}

Token get_conditional_operator_token(char* start, char* end){
  char* op = NULL;
  op = strchr(start, '<');
  if(op < end)
    return get_operator_token(op);
  op = strchr(start, '>');
  if(op < end)
    return get_operator_token(op);
  // TODO: below conditional operators
  // op = strchr(start, '<=');
  // if(op < end)
  //   return return_operator_token(op);
  // op = strchr(start, '>=');
  // if(op < end)
  //   return return_operator_token(op);
  // op = strchr(start, '==');
  // if(op < end)
  //   return return_operator_token(op);
}

// return 0 -> false, 1 -> true, 2 -> invalid
uint8_t evaluate_between_parenthesis(const char* begin, const char* end){
  if(begin != '(' || end != ')')
    return 2;

  // assuming sytax : if (3 > 4)
  char* start = begin+1;
  Token conditional_operator = get_conditional_operator_token(start, end);
  // TODO: can make a pair utility to return {Token, char*} the char* to the next operator
  char* op = strchr(start, conditional_operator.token_value.ch);
  char* first_token_end = op-1; // 1 for the space
  
  Token* t1;
  if(isdigit(start))
    t1 = get_literal_token(start, first_token_end-start);
  else
    t1 = get_identifier_token(start, first_token_end - start);

  Token* t2;
  if(isdigit(start))
    t2 = get_literal_token(op+1, end-1);
  else
    t2 = get_identifier_token(op+1, end-1);

  // evaluate
  if(conditional_operator.token_value.ch == '>'){
    double  v1 = t1.token_value.double_value, v2 = t2.token_value.double_value;
    return (v1 > v2) ? 1 : 0;
  }
  if(conditional_operator.token_value.ch == '<'){
    double  v1 = t1.token_value.double_value, v2 = t2.token_value.double_value;
    return (v1 < v2) ? 1 : 0;
  }
  // do more conditional operators
  return 2;
}
