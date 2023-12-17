#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define GND 0

typedef struct OpResult {
  double *data;
  uintptr_t len;
} OpResult;

struct OpResult solve_netlist(const char *input);
