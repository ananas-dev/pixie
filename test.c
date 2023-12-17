#include "pixie.h"
#include "stdio.h"

int main() {
    OpResult result = solve_netlist("V1 0 1 10\nD1 1 2 1e-12 300\nR1 2 0 100");

    printf("Results: [ ");

    for (int i = 0; i < result.len; i++) {
        printf("%.2f ", result.data[i]);
    }

    printf("]\n");

    free(result.data);
}