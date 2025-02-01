pub fn filtro(texto: &str) -> String {
    // Lista inicial com algumas palavras ofensivas reais
    let palavras_ofensivas: Vec<(&str, &str)> = vec![
        ("caralho", "*******"),
        ("carai", "*****"),
        ("puta", "****"),
        ("merda", "*****"),
        ("bosta", "*****"),
        ("foda", "****"),
        ("foder", "*****"),
        ("fudido", "*******"),
        ("fudida", "*******"),
        ("cacete", "******"),
        ("desgraça", "********"),
        ("babaca", "******"),
        ("cu", "***"),
        ("xota", "*****"),
        ("buceta", "*******"),
        ("viado", "******"),
        ("arrombado", "*********"),
        ("corno", "*****"),
        ("imbecil", "*******"),
        ("idiota", "******"),
        ("otário", "*******"),
        ("estúpido", "********"),
        ("cretino", "*******"),
        ("filho da puta", "***************"),
        ("desgraçado", "***********"),
        ("porra", "*****"),
        ("piranha", "*******"),
        ("vagabunda", "*********"),
        ("vagabundo", "*********"),
        ("chupa", "*****"),
        ("pica", "****"),
        ("pau", "***"),
        ("bucetudo", "*********"),
        ("sacanagem", "*********"),
        ("bicha", "******"),
    ];

    let mut texto_filtrado = texto.to_owned();
    // Para cada palavra ofensiva, substitui todas as ocorrências no texto.
    for (palavra, substituto) in palavras_ofensivas.iter() {
        texto_filtrado = texto_filtrado.replace(palavra, substituto);
    }
    return texto_filtrado;
}
