# Hero starter pack
Every good here starts out with some basic equipment! We are here to learn `Rust` after all.

Your mission is to get the control interface back to work. It is integral on which all following missions depend on. You are our last hope!

## Installation
1. Install the proper Rust toolchain.
    1. Install rustup and the rust compiler
        ```shell-script
        pacman -S rustup
        rustup install nightly
        rustup default nightly
        ```
    2. Install cargo
        ```shell-script
        pacman -S cargo
        ```
2. Install docker and docker-compose
    ```shell-script
    pacman -S docker docker-compose
    systemctl start docker
    ```
3. Install NPM
    ```shell-script
    pacman -S npm
    ```
4. Start the Environment
    ```shell-script
    cd Environment
    docker-compose up
    ```
5. Start the Webclient
    ```shell-script
    cd Webclient
    npm i
    npm run start
    ```
6. Lastly create the `backend` and get going!
    ```shell-script
    cargo new backend
    cd Backend
    code ./
    # Implement it
    cargo run
    ``` 