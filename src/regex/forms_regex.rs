use regex::Regex;

pub fn password_validate(password: &str) -> bool {
    // Verifica se a senha tem no mínimo 8 caracteres
    if password.len() < 8 {
        return false;
    }

    // Verifica se possui pelo menos uma letra maiúscula
    let re_upercase = Regex::new(r"[A-Z]").unwrap();
    if !re_upercase.is_match(password) {
        return false;
    }

    // Verifica se possui pelo menos um caractere especial
    let re_especial = Regex::new(r#"[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]"#).unwrap();
    if !re_especial.is_match(password) {
        return false;
    }

    true
}