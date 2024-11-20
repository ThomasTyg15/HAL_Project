---
# 🛠️ **my_hal**: A Custom Embedded Hardware Abstraction Layer (HAL)

Welcome our **hal**, a powerful yet simple **Hardware Abstraction Layer (HAL)** designed for embedded systems! 🚀 This library provides a unified API to work with peripherals like GPIO and USART across 2 microcontroller targets, including **ATmega328P** (Arduino) and **Cortex-M3** (STM32F103).

---

## 📂 **Project Structure**

Here's a quick overview of the project's organization:

```plaintext
my_hal/
│
├── Cargo.toml                  # Project metadata and dependencies
├── src/
│   ├── main.rs                # Usage example
│   ├── lib.rs                 # Main library export
│   │
│   ├── hal/                   # Core HAL functionality
│   │   ├── mod.rs             # HAL traits and shared types
│   │   └── target.rs          # Supported target enumeration
│   │
│   ├── gpio/                  # GPIO abstractions
│   │   ├── mod.rs             # GPIO module export
│   │   ├── traits.rs          # GPIO common traits
│   │   ├── atmega328p.rs      # GPIO implementation for ATmega328P
│   │   └── cortex_m3.rs       # GPIO implementation for Cortex-M3
│   │
│   ├── usart/                 # USART abstractions
│   │   ├── mod.rs             # USART module export
│   │   ├── traits.rs          # USART common traits
│   │   ├── atmega328p.rs      # USART implementation for ATmega328P
│   │   └── cortex_m3.rs       # USART implementation for Cortex-M3
│   │
│   └── drivers/               # Generic drivers using HAL
│       └── mod.rs             # Driver implementations
│
├── examples/                  # Target-specific examples
│   ├── atmega328p_blink.rs
│   ├── atmega328p_serial.rs
│   ├── cortex_m3_blink.rs
│   └── cortex_m3_serial.rs
│
└── README.md                  # This file!
```
---

## ✨ **Features**

- **🎛️ GPIO Abstraction:** Control GPIO pins with ease using a unified API.
- **📡 USART Support:** Send and receive data over serial interfaces.
- **🎯 Multiple Targets:** Write once, deploy on:
  - 🛠️ **ATmega328P** (Arduino)
  - 🔧 **Cortex-M3** (STM32F103)
- **📚 Extensible Design:** Add drivers for custom peripherals like sensors, LCDs, or actuators.
- **🚀 Ready-to-Use Examples:** Start hacking with pre-written examples for each target.

---

## 🏗️ **Getting Started**

To validate the functionality of the HAL, you have several options:

### 1️⃣ Compilation and Target-Specific Tests
Compile the project for different targets using feature flags:

```bash
# For ATmega328P
cargo build --target avr-atmega328p.json

# For Cortex-M3
cargo build --target thumbv7m-none-eabi

```

This ensures the HAL is correctly compiled for the intended hardware platform.

---

### 2️⃣ Unit Tests
Create a `tests/` directory in your project and include files for specific modules. For example:

#### GPIO Tests:
```rust
// tests/gpio_tests.rs
#[cfg(test)]
mod tests {
    use my_hal::gpio::{AtmegaPin, GpioPin, Direction};

    #[test]
    fn test_gpio_pin() {
        // Test GPIO functionality in a simulation/emulation setup
    }
}
```

#### USART Tests:
```rust
// tests/usart_tests.rs
#[cfg(test)]
mod tests {
    use my_hal::usart::{AtmegaUsart, UsartInterface};

    #[test]
    fn test_usart_init() {
        // Test USART initialization
    }
}
```

Run these tests using:
```bash
cargo test
```

---

### 3️⃣ Example Programs
Add sample programs to the `examples/` directory to demonstrate HAL functionality. For example:

- **Blink an LED** on ATmega328P:
  ```rust
  fn main() {
      // Example code for blinking an LED
  }
  ```

- **USART Communication** on Cortex-M3:
  ```rust
  fn main() {
      // Example USART communication setup
  }
  ```

Compile and flash the examples to the target hardware:
```bash
cargo run --example <example_name> --features=<target>
```

---

### 4️⃣ Simulation and Emulation
Simulate the HAL using tools like:

- **QEMU:** Run the HAL code in an emulated environment.
- **Embedded Frameworks:** Use frameworks like `embedded-hal-driver-tests` to validate HAL functionality.

For QEMU, configure the emulator to match the target hardware's architecture. For instance:
```bash
qemu-system-avr -M atmega328p -kernel target/atmega328p/debug/<binary>
```

---

### 5️⃣ Hardware Validation
For a complete validation, test on physical hardware:

- **ATmega328P:** Use an Arduino Uno or compatible board.
- **Cortex-M3:** Use an STM32 development board.

Flash the compiled binaries and observe the behavior to ensure HAL correctness.

---

## 🚦 **Examples**

Run the examples to see **my_hal** in action:
```bash
cargo run --example atmega328p_blink --features atmega328p
cargo run --example cortex_m3_blink --features stm32f1
```

---
