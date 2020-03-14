# Hero starter pack
Every good hero starts out with some basic equipment! We are here to learn `Rust` after all.

Your mission is to get the control interface back to work. It is integral on which all following missions depend on. You are our last hope!

## Installation
1. Start the Environment
    ```shell-script
    cd Environment
    docker-compose up
    ```
2. Start the Webclient
    ```shell-script
    cd Webclient
    npm i
    npm run start
    ```
3. Lastly create the `backend` and get going!
    ```shell-script
    cargo new backend
    cd backend
    code ./
    # Implement it
    cargo run
    ``` 

## Mission
1. Create the base structure:
    ```shell-script
    crate
    ├── tools
    │   └── ...
    ├── transfer
    │   └── ...
    ├── tests
    │   └── ...
    └── domain
        └── ...
    ```
2. Integrate the web framework [Rocket](https://rocket.rs/).
3. Implement an `Hello, World!` endpoint under `/API/`.
4. Model a structure that maintains a list of our heroes.
5. Let `Rocket` [manage](https://rocket.rs/v0.4/guide/state/#managed-state) your stucture.
6. Write an Initializing function for your structure that loads the list of heroes from the database.
    * You could use [this crate](https://docs.rs/mysql/18.0.0/mysql/) for example.
    * IP: localhost/127.0.0.1
    * Port: 3306
    * User: root/mysql
    * Password: vagrant
    * DB name: main
    * Table name: heroes
        * Columns:
            * id: u32
            * name: String
            * sub_title: Option<String>
            * strength: String
            * weakness: Option<string>
            * hero_call: Option<string>
7. Create a structure to model our `Hero`.
8. Initialize your structure before letting Rocket manage it.
9. Create an endpoint stub for `/API/heroes`
    * Method: GET
    * Input: The [state](https://rocket.rs/v0.4/guide/state/#retrieving-state) of your structure that maintains the heroes
    * Output: Json<Vec<Hero>>
    * Hint: You can use the `unimplemented!()` macro for stub return values
10. Create a trait `GetHeroes` that defines a function, which returns `Vec<Hero>`.
11. Implement a stub implementation for your structure of `GetHeroes`.
12. Replace `unimplemented!()` with a call to your `GetHeroes` implementation.
13. Implement it by simply returning a copy of your whole hero list.
14. Test your implementation by manually looking up: http://localhost:8000/API/heroes
15. The Frontend sends filter/sorting parameters to the backend.
    * The structure looks the following
        ```rust
        pub struct HeroSearchFilter {
            pub page: u32,
            pub id: TableFilter<u32>,
            pub name: TableFilter<String>,
            pub sub_title: TableFilter<String>,
            pub strength: TableFilter<String>,
            pub weakness: TableFilter<String>,
            pub hero_call: TableFilter<String>,
        }
        ```
        ```rust
        pub struct TableFilter<T> {
            pub filter: Option<T>,
            pub sorting: Option<bool>,
        }
        ```
    1. Modify your `get_heroes` endpoint:
        * Method: POST
        * Paramater: Json<HeroSearchFilter>
        * You may want to take a look at [this](https://rocket.rs/v0.4/guide/requests/#format).
    2. Modify your trait to also take the `HeroSearchFilter`.
    3. Now use the `HeroSearchFilter` object to filter and sort the your hero list.
16. Run the backend and test if the Frontend works: http://localhost:4200