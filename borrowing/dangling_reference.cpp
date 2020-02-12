#include <iostream>

int main() {
    int *answer;
    {
        int calc_answer = 2*3*7;
        answer = &calc_answer;
    }
    std::cout << "Address of answer: " << answer << std::endl;
    std::cout << "Answer is: " << *answer << std::endl;
}