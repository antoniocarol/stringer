# 🇧🇷 Stringer - Brazilian String Manipulation

The **Rust library** focused on Brazilian string operations with CPF/CNPJ validation and localized formatting.

## 🚀 Features

- ✅ **Memory-optimized** string operations (zero allocation when possible)
- ✅ **CPF/CNPJ validation** with Brazilian algorithms  
- ✅ **Brazilian formatting** (phone, CEP, currency)
- ✅ **Macro system** for easy interpolation (`br_format!`)
- ✅ **Unicode-safe** and performant

## 📖 Usage

```rust
use stringer::*;

// String operations
let trimmed = trim("  hello  ");        // "hello"
let upper = upper("rust");              // "RUST"

// Brazilian validations
let valid = is_valid_cpf("111.444.777-35");  // true
let valid = is_valid_cnpj("11.222.333/0001-81"); // true

// Brazilian formatting  
let phone = format_phone_br("11987654321");     // "(11) 98765-4321"
let cep = format_cep("01234567");               // "01234-567"

// Macro system
let price = br_format!(currency, 150.50);      // "R$ 150.50"
let doc = br_format!(cpf, "11144477735");      // "111.444.777-35"