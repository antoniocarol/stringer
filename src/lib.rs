pub mod stringer;
pub use stringer::*;

#[macro_export]
macro_rules! br_format {
    //FORMATA CURRENCY
    (currency, $value:expr) => {
        format!("R$ {:.2}", $value)
    };

    //FORMAT CPF
    (cpf, $cpf:expr) => {
        format!("{}.{}.{}-{}", &$cpf[0..3], &$cpf[3..6], &$cpf[6..9], &$cpf[9..11])
    };

    (cnpj, $cnpj:expr) => {
        format!("{}.{}.{}/{}-{}", 
        &$cnpj[0..2], &$cnpj[2..5], &$cnpj[5..8], 
        &$cnpj[8..12], &$cnpj[12..14])
    };

    (phone, $phone:expr) => {
        $crate::format_phone_br($phone)
    };

    (cep, $cep:expr) => {
        $crate::format_cep($cep)
    };

    ($($arg:tt)*) => {
        format! ($($arg)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trimmer(){
        assert_eq!(trim("  Hello, World!  "), "Hello, World!");
        assert_eq!(trim("Rust"), "Rust");
        assert_eq!(trim("   "), "");
        assert_eq!(trim(""), "");
        assert_eq!(trim("  Leading and trailing spaces  "), "Leading and trailing spaces");
    }

    #[test]
    fn test_uppercase() {
        assert_eq!(upper("hello"), "HELLO");
        assert_eq!(upper("Hello, World!"), "HELLO, WORLD!");
        assert_eq!(upper("Rust"), "RUST");
        assert_eq!(upper("123"), "123");
        assert_eq!(upper(""), "");
    }


    #[test]
    fn test_lowercase() { 
        assert_eq!(lower("HELLO"), "hello");
        assert_eq!(lower("HeLLo"), "hello");
        assert_eq!(lower("hello"), "hello");
        assert_eq!(lower("HeLlO"), "hello"); 
        assert_eq!(lower("123"), "123");
    }

    #[test]
    fn test_cpf_validation() {
        assert_eq!(is_valid_cpf("11144477735"), true);
        assert_eq!(is_valid_cpf("111.444.777-35"), true);
        
        // CPFs inválidos
        assert_eq!(is_valid_cpf("12345678901"), false);
        assert_eq!(is_valid_cpf("111.111.111-11"), false);
        assert_eq!(is_valid_cpf("123"), false);
        assert_eq!(is_valid_cpf(""), false);
    }

    #[test]
    fn test_br_format() {
        assert_eq!(br_format!(currency, 1500.50), "R$ 1500.50");
        assert_eq!(br_format!(cpf, "12345678901"), "123.456.789-01");
        assert_eq!(br_format!("Hello {}!", "Rust"), "Hello Rust!");
    }

    #[test]
    fn test_cnpj_validation() {
        // CNPJ válido conhecido
        assert_eq!(is_valid_cnpj("11222333000181"), true);
        assert_eq!(is_valid_cnpj("11.222.333/0001-81"), true);
        
        // CNPJs inválidos garantidos
        assert_eq!(is_valid_cnpj("11222333000180"), false); // Dígito final errado
        assert_eq!(is_valid_cnpj("00000000000000"), false); // Todos zeros
        assert_eq!(is_valid_cnpj("11111111111111"), false); // Todos iguais
        assert_eq!(is_valid_cnpj("123"), false);            // Muito curto
        assert_eq!(is_valid_cnpj(""), false);               // Vazio
    }

    #[test]
    fn test_format_functions() {
        assert_eq!(format_phone_br("11987654321"), "(11) 98765-4321");
        assert_eq!(format_phone_br("1187654321"), "(11) 8765-4321");
        assert_eq!(format_cep("01234567"), "01234-567");
        
        // Testes da macro expandida
        assert_eq!(br_format!(cnpj, "11222333000181"), "11.222.333/0001-81");
        assert_eq!(br_format!(phone, "11987654321"), "(11) 98765-4321");
        assert_eq!(br_format!(cep, "01234567"), "01234-567");
    }
}
