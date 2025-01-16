# Rocket Project Folder Structure

```plaintext
hello_rocket/
├── Cargo.toml                     # Project dependencies and metadata
├── Rocket.toml                    # Rocket configuration (e.g., database URL)
├── .env                           # Environment variables (for Diesel and Rocket)
├── migrations/                    # Diesel migration files
│   ├── 0001_create_users/
│   │   ├── up.sql                 # SQL to create the users table
│   │   └── down.sql               # SQL to drop the users table
│   └── README.md
├── src/
│   ├── main.rs                    # Rocket application entry point
│   ├── lib.rs                     # Centralizes module exports
│   ├── db/
│   │   ├── mod.rs                 # Database module (connection setup)
│   │   └── schema.rs              # Diesel schema file (auto-generated)
│   ├── entities/
│   │   ├── mod.rs                 # Entity module centralization
│   │   └── user.rs                # User entity and CRUD operations
│   ├── routes/
│   │   ├── mod.rs                 # Routes module centralization
│   │   └── user_routes.rs         # User-related routes
│   └── services/
│       ├── mod.rs                 # Service module centralization
│       └── user_service.rs        # Business logic for users
├── tests/                         # Integration tests
│   └── user_tests.rs
└── README.md                      # Project description and setup instructions
```
