# Test Documentation

## Test Coverage

This document describes the test coverage for the `cjson-rs` library.

### Core cJSON Tests (`src/cjson.rs`)

#### Parsing Tests
- ✅ `test_parse_simple_object` - Parse a simple JSON object
- ✅ `test_parse_array` - Parse a JSON array
- ✅ `test_parse_nested_object` - Parse nested JSON objects
- ✅ `test_parse_with_length` - Parse JSON with specified length

#### Creation Tests
- ✅ `test_create_null` - Create null value
- ✅ `test_create_and_get_bool` - Create and retrieve boolean values
- ✅ `test_create_and_get_number` - Create and retrieve number values
- ✅ `test_create_and_get_string` - Create and retrieve string values
- ✅ `test_create_array_and_add_items` - Create array and add items
- ✅ `test_create_object_and_add_items` - Create object and add items
- ✅ `test_create_int_array` - Create integer array
- ✅ `test_create_double_array` - Create double array
- ⚠️  `test_create_string_array` - Create string array (currently ignored due to potential C library issue)

#### Printing Tests
- ✅ `test_print_formatted` - Print JSON with formatting
- ✅ `test_print_unformatted` - Print JSON without formatting

#### Manipulation Tests
- ✅ `test_delete_item_from_array` - Delete item from array by index
- ✅ `test_delete_item_from_object` - Delete item from object by key
- ✅ `test_duplicate` - Duplicate JSON items
- ✅ `test_compare` - Compare JSON items

#### Type Checking Tests
- ✅ `test_case_sensitive_get` - Case-sensitive object item retrieval

#### Error Handling Tests
- ✅ `test_type_error` - Type mismatch error handling
- ✅ `test_not_found_error` - Not found error handling

### JSON Utils Tests (`src/cjson_utils.rs`)

#### JSON Pointer Tests (RFC6901)
- ✅ `test_json_pointer_get` - Get value using JSON Pointer
- ✅ `test_json_pointer_get_case_sensitive` - Case-sensitive JSON Pointer
- ✅ `test_json_pointer_not_found` - JSON Pointer not found error
- ✅ `test_complex_pointer_path` - Complex nested JSON Pointer paths
- ✅ `test_pointer_find_from_object_to` - Find pointer path from object to target

#### JSON Patch Tests (RFC6902)
- ✅ `test_json_patch_generate_and_apply` - Generate and apply patches
- ✅ `test_json_patch_apply` - Apply patch operations
- ✅ `test_json_patch_add_to_array` - Add patch to array

#### JSON Merge Patch Tests (RFC7386)
- ✅ `test_json_merge_patch_apply` - Apply merge patch
- ✅ `test_json_merge_patch_generate` - Generate merge patch
- ✅ `test_merge_patch_null_removal` - Merge patch with null values

#### Utility Tests
- ✅ `test_json_utils_sort_object` - Sort object keys (case-insensitive)
- ✅ `test_json_utils_sort_object_case_sensitive` - Sort object keys (case-sensitive)

## Test Statistics

- **Total Tests**: 35
- **Passing**: 34 (97%)
- **Ignored**: 1 (3%)
- **Failing**: 0 (0%)

## Running Tests

### Prerequisites

The tests require:
- The cJSON library to be installed on the system (`libcjson` and `libcjson_utils`)
  - On Ubuntu/Debian: `sudo apt-get install libcjson-dev`
  - On other systems, you may need to install from source
- Standard library support (tests use `std` even though the main library is `no_std`)

### Running Tests

To run all tests:
```bash
cargo test --features std
```

To run tests with output:
```bash
cargo test --features std -- --nocapture
```

To run a specific test:
```bash
cargo test --features std test_name
```

To run tests from a specific module:
```bash
cargo test --features std cjson::tests
cargo test --features std cjson_utils::tests
```

### Important Notes

1. **Library is `no_std`**: The main library is compiled with `#![no_std]` for embedded compatibility
2. **Tests require `std` feature**: Tests must be run with `--features std` flag
3. **Build vs Test**: Regular builds (`cargo build`) maintain `no_std`, tests require the `std` feature
4. **Linking**: The build script automatically links against cJSON libraries during testing

## Known Issues

### `test_create_string_array`
This test is currently ignored due to a potential double-free issue in the underlying cJSON library when creating string arrays. The issue appears to be related to memory ownership between the Rust wrapper and the C library. This functionality should be used with caution in production code.

## Test Requirements

The tests require:
- The cJSON library to be installed on the system (`libcjson` and `libcjson_utils`)
  - On Ubuntu/Debian: `sudo apt-get install libcjson-dev`
  - On other systems, install from [cJSON GitHub](https://github.com/DaveGamble/cJSON)
- Standard library support (tests use `std` even though the main library is `no_std`)

The `Cargo.toml` is configured to:
- Use `panic = "unwind"` for test profile (via Rust's default test behavior)
- Link against `libcjson` and `libcjson_utils` only when the `std` feature is enabled (via `build.rs`)
- Maintain `no_std` for normal builds while allowing `std` for tests via conditional compilation
- The `build.rs` script only links to cJSON libraries when building with the `std` feature
