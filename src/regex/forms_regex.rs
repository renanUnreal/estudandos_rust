use regex::Regex;

pub fn validar_senha(senha: &str) -> bool {
    // Verifica se a senha tem no mínimo 8 caracteres
    if senha.len() < 8 {
        return false;
    }

    // Verifica se possui pelo menos uma letra maiúscula
    let re_maiuscula = Regex::new(r"[A-Z]").unwrap();
    if !re_maiuscula.is_match(senha) {
        return false;
    }

    // Verifica se possui pelo menos um caractere especial
    let re_especial = Regex::new(r#"[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]"#).unwrap();
    if !re_especial.is_match(senha) {
        return false;
    }

    true
}