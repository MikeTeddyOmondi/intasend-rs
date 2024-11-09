# Available Module Imports

## Core Module

| Module Name          | Description                | Key Types/Structs | Common Uses                                              |
| -------------------- | -------------------------- | ----------------- | -------------------------------------------------------- |
| `intasend::Intasend` | Core client implementation | `Intasend`        | Creating IntaSend API client and managing authentication |

## Utility Modules

| Module Name          | Description         | Key Types/Structs | Common Uses                      |
| -------------------- | ------------------- | ----------------- | -------------------------------- |
| `intasend::Provider` | Supported Providers | `Provider`        | Specifying which Provider to use |

## Notes

- All modules are available under the main `intasend` crate
- Import specific types using fully qualified paths
- Error handling is done via `Result<T, IntaSendApiError>`
- Configuration can be environment-specific
- Most operations require authentication via the client

## Example Usage

_coming soon_

## Common Import Patterns

_coming soon_
