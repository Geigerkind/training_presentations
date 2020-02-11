#include <iostream>
#include <string>

class Dog {
    std::string mName;
public:
    Dog(std::string &&name) {
        mName = std::move(name);
    }

    void bark() {
        std::cout << "Wuff!" << std::endl;
    }
};

int main() {
    std::string name = "Snuffles";
    Dog *snuffles = new Dog(std::move(name));

    delete snuffles; // RIP
    
    snuffles->bark();
}