#include <iostream>

int* dangling() {
    int *a = (int*)malloc(sizeof(int));
    *a = 2*3*7;
    delete a;
    return a;
}

int main() {
    int *answer = dangling();
    std::cout << "Address of answer: " << answer << std::endl;
    std::cout << "Answer is: " << *answer << std::endl;
}