---

# **HAL Project in Rust**

## **Project Overview**
This project implements a **Hardware Abstraction Layer (HAL)** in **Rust** for two microcontrollers:
1. **Atmega328p** (Arduino Uno)
2. **Cortex-M3** (e.g., STM32 series)

The goal of this project is to provide a unified and simple interface for controlling hardware peripherals, regardless of the underlying microcontroller. This means that users can interact with **GPIO**, **USART**, and **SPI** functionalities without needing to know the specifics of the microcontroller's registers.

## **Features**
- **General-Purpose Input/Output (GPIO):**
  - Configure pins as **input** or **output**.
  - **Read** and **write** to digital pins.
  - Example: Turn an LED on/off or read the state of a button.

- **Universal Synchronous/Asynchronous Receiver/Transmitter (USART):**
  - Initialize communication with a configurable baud rate.
  - **Send** and **receive** data over a serial interface.
  - Example: Communicate with a PC or another microcontroller.

- **Serial Peripheral Interface (SPI):**
  - Master mode operation with configurable clock speed.
  - Perform **data transfers** to and from SPI peripherals.
  - Example: Interact with an SPI sensor or memory module.

## **Project Structure**
The project is modular and organized as follows:
```
my_hal_project/
├── Cargo.toml           # Rust project configuration
├── src/
│   ├── main.rs          # Entry point with usage examples
│   ├── lib.rs           # Exports all modules
│   ├── gpio/            # GPIO module
│   │   ├── mod.rs       # Interface for GPIO
│   │   ├── atmega328p.rs # GPIO implementation for Atmega328p
│   │   └── cortex_m3.rs # GPIO implementation for Cortex-M3
│   ├── usart/           # USART module
│   │   ├── mod.rs       # Interface for USART
│   │   ├── atmega328p.rs # USART implementation for Atmega328p
│   │   └── cortex_m3.rs # USART implementation for Cortex-M3
│   ├── spi/             # SPI module
│   │   ├── mod.rs       # Interface for SPI
│   │   ├── atmega328p.rs # SPI implementation for Atmega328p
│   │   └── cortex_m3.rs # SPI implementation for Cortex-M3
├   ├──I2C/           # I2C module
│       ├── mod.rs       # Interface for I2C
│       ├── atmega328p.rs # I2C implementation for Atmega328p
│       └── cortex_m3.rs # I2C implementation for Cortex-M3

```

## **Getting Started**
### **1. Requirements**
- A Rust compiler (Install with `rustup`):  
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Microcontroller tools, such as:
  - `avr-gcc` and `avrdude` for Atmega328p
  - `openocd` and `arm-none-eabi-gcc` for Cortex-M3

### **2. Build the Project**
To compile the project for a specific microcontroller, use Rust's feature flags:

- For **Atmega328p**:
  ```bash
  cargo build --features atmega328p
  ```

- For **Cortex-M3**:
  ```bash
  cargo build --features cortex_m3 --target thumbv7m-none-eabi
  ```

### **3. Flash the Firmware**
- **For Atmega328p (Arduino Uno)**, use `avrdude` to flash the generated `.hex` file:
  ```bash
  avrdude -c arduino -p m328p -P /dev/ttyUSB0 -b 115200 -U flash:w:target.hex:i
  ```
  
- **For Cortex-M3** (e.g., STM32), use `openocd` to flash the firmware:
  ```bash
  openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg -c "program target.hex verify reset exit"
  ```

## **Verifying Functionality**
To verify that the project is working, you can:
1. **GPIO**: Use an LED to check if GPIO pins are configured and can switch on/off.
2. **USART**: Open a terminal to check if data is being transmitted and received via the serial port.
3. **SPI**: Use an external SPI device (like an SPI sensor) and check if the communication is successful.

### **5. Running Tests with `cargo test`**
You can run unit tests (in software) to check the logic behind the GPIO, USART, and SPI modules:
- **For Atmega328p**:
  ```bash
  cargo test --features atmega328p
  ```

- **For Cortex-M3**:
  ```bash
  cargo test --features cortex_m3
  ```

These tests will verify that the logic of configuring pins, reading and writing data, and sending/receiving messages is correct.





[CORRECTION SPI] (don't hesitate to remove this part)
You should implement the peripheral/slave mode as well (not only the controler/master mode).
You could abstract more the register content, for example ```(1 << 6) | (1 << 4) | (1 << 1)``` is not very explicit, you may want to customize your parameters more accurately (therefore you could use more freely all the part of your registers (CPOL, CPHA, BR, MSTR...)).