#include <iostream>
#include <string>

class Dog {
    std::string mName;
public:
    Dog(const char* name) {
        mName = std::move(name);
    }

    void bark() {
        std::cout << "Wuff!" << std::endl;
    }
};

int main() {
    Dog *snuffles = new Dog("Snuffles");

    delete snuffles; // RIP
    
    snuffles->bark();
}