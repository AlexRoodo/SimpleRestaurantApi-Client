# Simple Restaurant Client

## Introduction
The project is developed for the [interview problem](https://github.com/paidy/interview/blob/master/SimpleRestaurantApi.md) of [Paidy Inc.](https://paidy.com/), in order to realize a simple food ordering system in a restaurant.
This is the client application for the project.

## Requirements for client
1. The client (the restaurant staff “devices” making the requests) MUST be able to: add one or more items with a table number, remove an item for a table, and query the items still remaining for a table.
2. The client MAY limit the number of specific tables in its requests to a finite set (at least 100).

Also:
- “Clients” can be simulated as simple threads in a main() function calling the main server application with a variety of requests. There should be more than one, preferably around 5-10 running at any one time.
- "Please make this as simple as possible."

## Project structure
#### generatots.rs
Contains utility functions to generate random values of strings and numbers
#### requests.rs
Contains collection of functions to perform valid operations on the server. Also contains an Enum for set of such operations
#### random.rs
Contains a function to generate a randomized call to the server from a set of valid operations and prints the report to the console
#### main.rs
Imitates a restaurant with a set of tablets and performs arbitrary operations on the server.

- From requirements, the number of tablets can be limited to a finite set. In this case, the number of tablets is equal to amount of service staff in the restaurant and declared in constant `TABLETS_COUNT` with value 15 (from requirements that number could be 5-10). Each stuff member can work in parallel with others. That feature is expressed through ThreadPool with num_threads equals `TABLETS_COUNT`.

- From requiements, limitations of the number of tables is at least 100. That limitation is expressed through constant TABLES_AMOUNT with value 200.

- Also there is defined constant `TASK_AMOUNT` for limiting whole number of tasks.

## Prerequisites

Rust and Cargo must be installed on the system
Simple Restaurant Api(server) must be running on the system

To build and run this application, apply theses commands in the project folder:

'''
cargo build --release
cargo run --bin restaurant_client
'''

## License

MIT
