#include <iostream>
#include <string>

int main() {
    std::string dog1 = "Snuffels";
    std::string dog2 = std::move(dog1);

    std::cout << "Dog 1: " << dog1 << " (" << &dog1 << ") " << std::endl;
    std::cout << "Dog 2: " << dog2 << " (" << &dog2 << ") " << std::endl;
}