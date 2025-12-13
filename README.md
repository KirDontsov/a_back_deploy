# Avito Accounts Controller with Diesel ORM

This project implements a controller for managing Avito accounts using Diesel ORM instead of SQLX. The implementation follows the same patterns as the existing users controller but includes additional functionality for encrypting and decrypting sensitive data.

## Features

- **CRUD Operations**: Create, Read, Update, and Delete Avito accounts
- **Encryption**: AES-256 encryption for sensitive data (client secrets and client IDs)
- **Diesel ORM**: Uses Diesel ORM for database operations instead of SQLX
- **JWT Authentication**: Protected endpoints that require valid JWT tokens

## Database Schema

The `avito_accounts` table has the following structure:

- `account_id`: UUID (Primary Key)
- `user_id`: UUID (Foreign Key to users table)
- `client_id`: VARCHAR (Avito client ID)
- `avito_client_secret`: TEXT (Encrypted Avito client secret)
- `avito_client_id`: TEXT (Encrypted Avito client ID)
- `is_connected`: BOOLEAN (Connection status)
- `created_ts`: TIMESTAMP (Creation timestamp)
- `updated_ts`: TIMESTAMP (Update timestamp)

## API Endpoints

- `GET /api/avito/accounts` - Get all Avito accounts for the authenticated user
- `GET /api/avito/accounts/{id}` - Get a specific Avito account by ID
- `POST /api/avito/accounts` - Create a new Avito account
- `PUT /api/avito/accounts/{id}` - Update an existing Avito account
- `DELETE /api/avito/accounts/{id}` - Delete an Avito account

## Encryption

Sensitive data (client secrets and client IDs) are encrypted using AES-256 in CBC mode with a randomly generated IV for each encryption operation. The IV is stored alongside the encrypted data in the format: `{IV_HEX}:{ENCRYPTED_DATA}`.

## Dependencies

- `aes = "0.8"`
- `cbc = { version = "0.1", features = ["std"] }`
- `hex = "0.4"`
- `rand_core = { version = "0.6", features = ["std"] }`
- `diesel = { version = "2.0", features = ["postgres", "r2d2", "uuid", "chrono"] }`

## Implementation Details

### Models

The implementation includes three main models:
- `AvitoAccount`: Represents a complete Avito account record
- `CreateAvitoAccount`: Used for creating new accounts
- `UpdateAvitoAccount`: Used for updating existing accounts

### Controllers

Each CRUD operation has its own controller file:
- `get_all_avito_accounts.rs`
- `get_avito_account_by_id.rs`
- `create_avito_account.rs`
- `update_avito_account.rs`
- `delete_avito_account.rs`

### Encryption Utilities

The encryption utilities are located in `src/utils/encryption.rs` and provide functions for:
- Encrypting data with AES-256-CBC
- Decrypting data with AES-256-CBC
- Generating random IVs
- Combining and splitting IVs from encrypted data
- Decrypting Avito credentials specifically

## Usage

To use the Avito accounts controller, ensure your application is configured with the necessary database connection and JWT authentication middleware. The endpoints will be available under the `/api/avito/accounts` path.

## Security Considerations

- The encryption key is currently hardcoded in the application. In a production environment, this should be stored securely (e.g., in environment variables or a secure key management system).
- Always validate user input before processing
- Ensure that only authorized users can access and modify their own Avito accounts