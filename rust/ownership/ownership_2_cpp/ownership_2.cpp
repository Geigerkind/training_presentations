#include <iostream>
#include <string>

void say_name(std::string &&name) {
    std::cout << name << std::endl;
}

int main() {
    std::string *dog1 = new std::string("Snuffels");
    std::string *dog2 = dog1;

    say_name(std::move(*dog1));
    say_name(std::move(*dog2));

    std::cout << "Dog 1: " << *dog1 << " (" << &dog1 << ") " << std::endl;
    std::cout << "Dog 2: " << *dog2 << " (" << &dog2 << ") " << std::endl;

    delete dog1;
}