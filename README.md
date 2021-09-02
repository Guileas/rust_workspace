# RUST API ROCKET WORKSPACE

**Copy the env.dist in a .env file and fill the var with your project information**

**Change the container name in makefile**

**To find you user UID and GID type in your terminal :** ```id```

1 - Build container

```
make build
```

2 - Start container
```
make start
```

3 - Open a terminal to work on

```
make init
```

4 - Launch cargo project setup

```
cargo init
```

5 - Start project (inside container)

```
cargo run
```

Test
```
cargo test
```

Directory structure:

📦src
 ┣ 📂db (store db connection pool, etc.)
 ┣ 📂models (data you store in your db)
 ┣ 📂requests (data sent to a route)
 ┣ 📂responses (complete response sent to the frontend,status code,resources,etc.)
 ┃ ┗ 📂resources (data sent to the frontend)
 ┣ 📂route (all your route, 1 folder/api version)
 ┃ ┗ 📂v1
 ┗ 📜main.rs
