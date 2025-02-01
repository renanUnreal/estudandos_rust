pub fn print_header(title : &str){
    let border = "====================================";
    println!("{}", border);
    // Centraliza o título em 36 espaços (pode ajustar conforme necessário)
    println!("{:^36}", title.to_uppercase());
    println!("{}", border);
}
pub fn print_footer(title : &str){
    let border = "====================================";
    println!("{}", border);
    // Centraliza o título em 36 espaços (pode ajustar conforme necessário)
    println!("{:^36}", title.to_uppercase());
    println!("{}", border);
}
pub fn print_menu_home(){
    let menu_txt = r#"
    1 - Gerengiar clientes
    2 - Gerenciar contas
    3 - Gerenciar agencias
    4 - Gerenciar caixas
    5 - sair
    "#;

    println!("{:36}", menu_txt.to_uppercase());
}
