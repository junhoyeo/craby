# Code Generation

This guide explains how Craby's code generation works and what files are generated.

## Overview

Craby analyzes your TypeScript NativeModule specs and automatically generates:

1. **Rust trait definitions** - Interface your implementation must follow
2. **Rust implementation templates** - Boilerplate for your module struct
3. **FFI layer** - Rust-to-C++ bridging code using cxx
4. **C++ bridge code** - Pure C++ TurboModule implementation
5. **Native build configs** - CMake for Android, XCFramework setup for iOS

## Running Code Generation

The primary command for code generation is:

```bash
npx crabygen
```

This command:
1. Scans your source directory for NativeModule specs
2. Analyzes the TypeScript AST
3. Generates corresponding Rust and C++ code

## Generated Files

### `crates/lib/src/module_impl.rs`

Your implementation file. This is where you write your actual Rust logic.

::: info

The default implementation file is only generated once to prevent overwriting your custom code. You can always reference the template in the `.craby` folder at your project root if needed.

:::

**Generated template:**

```rust
use crate::ffi::bridging::*;
use crate::generated::*;
use crate::types::*;

pub struct Calculator {
    id: usize,
}

impl CalculatorSpec for Calculator {
    fn new(id: usize) -> Self {
        Calculator { id }
    }

    fn id(&self) -> usize {
        self.id
    }

    fn add(&mut self, a: Number, b: Number) -> Number {
        unimplemented!();
    }

    fn greet(&mut self, name: String) -> String {
        unimplemented!();
    }
}
```

## Understanding Generated Code

### Type Mapping

Craby automatically maps TypeScript types to Rust types during code generation. For detailed information about supported types and their mappings, see the [Types](/guide/types) guide.

### Signal Generation

Signals are special. For a TypeScript spec with signals:

```typescript
export interface Spec extends NativeModule {
  onData: Signal;
  onError: Signal;
}
```

Craby generates:

```rust
pub enum MyModuleSignal {
    OnData,
    OnError,
}

// In your impl:
impl MyModuleSpec for MyModule {
    fn some_method(&mut self) {
        self.emit(MyModuleSignal::OnData);
    }
}
```

### Naming Conventions

Method and field names are automatically converted to `snake_case`:

| TypeScript | Rust |
|------------|------|
| `getUserName` | `get_user_name` |
| `isActive` | `is_active` |
| `phoneNumber` | `phone_number` |

### Preview Generated Specs

See what modules and methods were detected:

```bash
npx crabygen show
```
