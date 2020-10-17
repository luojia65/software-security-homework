void use_buf3(unsigned int size) {
    if (size >= 128) {
        return;
    }

    int *A = malloc(sizeof(int)*size);
    // use A
    free(A);
}

void use_buf_4() {
    unsigned int size2 = 200;
    if (size2 >= 128) {
        return;
    }

    int *A = malloc(sizeof(int)*size2);
    // use A
    free(A);
}
