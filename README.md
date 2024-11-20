```markdown
# 🛠️ **my_hal**: A Custom Embedded Hardware Abstraction Layer (HAL)

Welcome to **my_hal**, a powerful yet simple **Hardware Abstraction Layer (HAL)** designed for embedded systems! 🚀 This library provides a unified API to work with peripherals like GPIO and USART across multiple microcontroller targets, including **ATmega328P** (Arduino) and **Cortex-M3** (STM32F103).

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

### 1️⃣ Add **my_hal** to your Project
In your `Cargo.toml`:
```toml
[dependencies]
my_hal = { path = "./path/to/my_hal" }
```

### 2️⃣ Select Your Target
Activate the appropriate feature:
- `atmega328p` for Arduino-like targets
- `stm32f1` for STM32F103

```toml
[features]
default = ["atmega328p"]
atmega328p = []
stm32f1 = ["dep:stm32f1"]
```

### 3️⃣ Use the Library
Here's an example for toggling a GPIO pin:

```rust
use my_hal::gpio::{AtmegaPin, Direction, PinMode};

fn main() {
    let mut pin = AtmegaPin::new(unsafe { &avr_device::atmega328p::PORTB }, 0);

    pin.set_direction(Direction::Output).unwrap();
    pin.set_high().unwrap();
    pin.set_low().unwrap();
}
```

---

## 🚦 **Examples**

Run the examples to see **my_hal** in action:
```bash
cargo run --example atmega328p_blink --features atmega328p
cargo run --example cortex_m3_blink --features stm32f1
```

---
