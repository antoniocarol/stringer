pub fn trim(input: &str) -> String {
   let trimmed = input.trim();
   if trimmed == input {
        input.to_string()
   } else {
        trimmed.to_string()
   }
}

pub fn upper(input: &str) -> String {
    let upper = input.to_uppercase();
    if upper == input {
        input.to_string()
    } else {
        upper.to_string()
    }
}

pub fn lower(input: &str) -> String {
    let lower = input.to_lowercase();
    if lower == input {
        input.to_string()
    } else {
        lower.to_string()
    }
}

pub fn is_valid_cpf(cpf: &str) -> bool {
    let clean_cpf: String = cpf.chars()
    .filter(|c| c.is_numeric())
    .collect();

    if clean_cpf.len() != 11 {
        return false;
    }

    let first_char = clean_cpf.chars().next().unwrap();
    if clean_cpf.chars().all(|c| c == first_char){
        return false;
    }

    let digits: Vec<u32> = clean_cpf.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut sum = 0;
    for i in 0..9 {
        sum += digits[i] * (10 - i as u32);
    }
    let first_digit = if sum % 11 < 2 { 0 } else { 11 - (sum % 11) };
    
    sum = 0;
    for i in 0..10 {
        sum += digits[i] * (11 - i as u32);
    }
    let second_digit = if sum % 11 < 2 { 0 } else { 11 - (sum % 11) };
    
    digits[9] == first_digit && digits[10] == second_digit
}

pub fn is_valid_cnpj(cnpj: &str) -> bool {
    let clean_cnpj: String = cnpj.chars().filter(|c| c.is_numeric()).collect();

    if clean_cnpj.len() != 14 {
        return false;
    }

    let first_char= clean_cnpj.chars().next().unwrap();
    if clean_cnpj.chars().all(|c| c == first_char) {
        return false;
    }

    let digits: Vec<u32> = clean_cnpj.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let weights1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut sum = 0;
    for i in 0..12 {
        sum += digits[i] * weights1[i];
    }
    let first_digit = if sum % 11 < 2 { 0 } else { 11 - (sum % 11) };

    // SEGUNDO DÍGITO VERIFICADOR  
    let weights2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    sum = 0;
    for i in 0..13 {
        sum += digits[i] * weights2[i];
    }
    let second_digit = if sum % 11 < 2 { 0 } else { 11 - (sum % 11) };

    // VERIFICAÇÃO FINAL
    digits[12] == first_digit && digits[13] == second_digit
}

pub fn format_phone_br(phone: &str) -> String {
    let clean: String = phone.chars().filter(|c| c.is_numeric()).collect();
    if clean.len() == 13 {
        format!("+{} ({}) {}-{}", &clean[0..2], &clean[2..4], &clean[4..9], &clean[9..13])
    } else  if clean.len() == 11 {
        format!("({}) {}-{}", &clean[0..2], &clean[2..7], &clean[7..11])
    } else if clean.len() == 10 {
        format!("({}) {}-{}", &clean[0..2], &clean[2..6], &clean[6..10])
    } else {
        phone.to_string()
    }
}

pub fn format_cep(cep: &str) -> String {
    let clean: String = cep.chars().filter(|c| c.is_numeric()).collect();
    if clean.len() == 8 {
        format!("{}-{}", &clean[0..5], &clean[5..8])
    } else {
        cep.to_string()
    }
}