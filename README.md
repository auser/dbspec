# Db Introspection Crate

A database introspection and generator library.

## Overview

This crate gives you the ability to create fake data for your living live database. 

## Features

- Connect to PostgreSQL or MySQL databases using asynchronous operations.
- Retrieve detailed information about tables including their names, types, and schemas.
- Fetch column details such as name, data type, and nullability.
- Generate fake data with a simple macro

## Usage

To use this crate, ensure you have a PostgreSQL or MySQL database running and accessible. You can specify the connection string to your database when running the main application, which utilizes this crate to fetch and display the database schema information.

### Library usage

### Fetching columns and their types

To get a list of all your tables, use the `get_tables()` function:

```rust
use dbspec::get_tables;

#[tokio::main]
async fn main() {
    let all_my_tables = get_tables("connection_string").await;
    // ...
}
```

You'll have the ability to generate fake data by using the `faking!()` macro.

```rust
use dbspec::faking;
use dbspec::fakeit; // or your choice of faking libraries

faking!(
    Id, id: String, || unique::uuid_v4();
    CreatedAt, created_at: chrono::DateTime<Utc>, random_datetime;
    UpdatedAt, updated_at: chrono::DateTime<Utc>, random_datetime;
    Gender, gender: String, || gender();
    Age, age: i32, || random_number_between(18, 100);
);

fn random_number_between(start: i32, end: i32) -> i32 {
    rand::thread_rng().gen_range(start..=end)
}
fn random_datetime() -> DateTime<Utc> {
    let data = datetime::date();
    DateTime::from_timestamp(data.secs, data.nsecs).expect("invalid or out-of-range datetime")
}
```

Where this really comes in handy is mapping columns to get a random value of your design, for instance:

```rust
async fn generate_new_user() {
    for i in 0..10 {
        let user = User {
            id: match_struct("id"),
            username: match_struct("username"),
        }
    }
}
```

