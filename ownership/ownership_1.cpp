#include <iostream>
#include <string>

int main() {
    std::string *dog1 = new std::string("Snuffels");
    std::string *dog2 = dog1;

    std::cout << "Dog 1: " << *dog1 << " (" << &dog1 << ") " << std::endl;
    std::cout << "Dog 2: " << *dog2 << " (" << &dog2 << ") " << std::endl;

    delete dog1;
}