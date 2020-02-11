# Rust as an alternative to C++
![CPP Risiken](img/cpp_leg_blowup.jpg)

## Use-After-Free
```c++
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
```

## Buffer overflow
```c++
#include <iostream>
#include <cstring>

int main() {
    int auth = 0;
    char password[10];
    std::cout << "Password: ";
    std::cin >> password;

    if (strcmp(password, "secret!") == 0) {
        auth = 1;
    } else {
        std::cout << "Wrong password!" << std::endl;
    }

    if (auth) {
        std::cout << "Logged in!" << std::endl;
    }
}
```

## Type Safe Languages
> Practitioners who invented type safety often meant just **“memory integrity”**, while theoreticians always meant **“execution integrity”**, and it is the latter that seems more relevant now
* Static and runtime checks
* Array bounds checking
* Garbage collection