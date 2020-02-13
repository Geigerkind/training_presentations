#include <iostream>
#include <memory>

int* dangling() {
    auto a = std::make_unique<int>(2*3*7);
    return a.get();
}

int main() {
    int *answer = dangling();
    std::cout << "Address of answer: " << answer << std::endl;
    std::cout << "Answer is: " << *answer << std::endl;
}