#include "zk_proof.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    printf("ZK Proof System C Interface Example\n");
    
    // Example input data
    uint8_t input[] = {1, 2, 3, 4, 5};
    size_t input_len = sizeof(input);
    
    // Allocate output buffer
    uint8_t output[1024];
    size_t output_len = sizeof(output);
    
    // Create proof
    int32_t result = zk_proof_create(input, input_len, output, &output_len);
    
    if (result == 0) {
        printf("Proof created successfully, size: %zu bytes\n", output_len);
        
        // Verify proof
        result = zk_proof_verify(output, output_len);
        
        if (result == 0) {
            printf("Proof verified successfully!\n");
        } else {
            printf("Proof verification failed\n");
        }
    } else {
        printf("Failed to create proof\n");
    }
    
    return 0;
}
