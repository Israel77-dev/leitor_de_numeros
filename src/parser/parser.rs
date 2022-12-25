/// Parser responsável por gerar uma sequência de caracteres correspondente às
/// leituras dos números em português.

// Gramática dos números em português, para leitura dos números entre 0 e 999 999 999 999 999 999 999 (classe dos quintilhões):
// número ::= [classe] [sufixo-de-classe];
// classe ::= "zero" | [dez-a-dezenove] | ([centena] "e" [dezena] "e" unidade) ;
// sufixo-de-classe ::= "mil" | "milhão" | "milhões" | "bilhão" | "bilhões" | "trilhão" | "trilhões" | "quatrilhão" | "quatrilhões" | "quintilhão" | "quintilhões";
// centena ::= "cem" | "duzentos" | "trezentos" | "quatrocentos" | "quinhentos" | "seicentos" | "setecentos" | "oitocentos" | "novecentos";
// dezena ::= "vinte" | "trinta" | "quarenta" | "cinquenta" | "sessenta" | "setenta" | "oitenta" | "noventa";
// dez-a-dezenove ::= "dez" | "onze" | "doze" | "treze" | "catorze" | "quinze" | "dezesseis" | "dezessete" | "dezoito" | "dezenove";
// unidade ::=  "um" | "dois" | "três" | "quatro" | "cinco" | "seis" | "sete" | "oito" | "nove";

/// Recebe um número entre 0 e 999 999 999 999 999 999 999 (classe dos quintilhões)
/// e retorna sua leitura em português.
///
/// Exemplo:
/// ```
/// # use leitor_de_numeros::parser::parser::ler_numero;
/// assert_eq!(ler_numero(0).unwrap(), "zero");
/// assert_eq!(ler_numero(7).unwrap(), "sete");
/// assert_eq!(ler_numero(1232).unwrap(), "mil duzentos e trinta e dois");
/// assert_eq!(ler_numero(10375).unwrap(), "dez mil trezentos e setenta e cinco");
/// assert_eq!(
///     ler_numero(2693412).unwrap(),
///     "dois milhões seiscentos e noventa e três mil quatrocentos e doze"
/// );
/// assert_eq!(ler_numero(3200).unwrap(), "três mil e duzentos");
/// assert_eq!(ler_numero(1035).unwrap(), "mil e trinta e cinco");
/// assert_eq!(ler_numero(1_000_035).unwrap(), "um milhão e trinta e cinco");
/// assert_eq!(
///     ler_numero(3_465_000).unwrap(),
///     "três milhões quatrocentos e sessenta e cinco mil"
/// );
/// ```
pub fn ler_numero(numero: u128) -> Option<String> {
    if numero > 999_999_999_999_999_999_999 {
        return None;
    }

    let classes = separar_classes(numero);
    let mut leitura = String::new();
    let num_classes = classes.len();
    let mut id_classe = (classes.len() - 1) as u8;

    for valor_classe in classes {
        leitura.push_str(&match (valor_classe, id_classe, num_classes) {
            // Número zero
            (0, 0, 1) => "zero".to_owned(),
            // Classe nula não é lida
            (0, _, _) => "".to_owned(),
            // 1000, é lido apenas como mil e não um mil
            (1, 1, _) => "mil ".to_owned(),
            // Classe das unidades simples (sem sufixo)
            (v, 0, 1) => ler_classe(v),
            // Classe das unidades simples deve ser prefixada com "e" caso seja menor que uma centena
            // ou quando o número termina em uma centena.
            (v, 0, _) if (v < 100 || v % 100 == 0) => format!("e {}", ler_classe(v)),
            // Caso singular (exceto mil)
            (1, i, _) => format!("um{}", sufixo_classe(i, false)),
            // Demais casos (plural)
            (v, i, _) => format!("{}{}", ler_classe(v), sufixo_classe(i, true)),
        });

        if id_classe > 0 {
            id_classe -= 1;
        }
    }

    Some(leitura.trim_end().to_owned())
}

/// Retorna o sufixo de classe baseado em um id (0 para unidades simples, 1 para milhares, etc.).
/// Para facilitar a geração da leitura dos numerais, o sufixo é delimitado por um espaço antes e
/// depois da palavra.
fn sufixo_classe(id: u8, plural: bool) -> &'static str {
    match (id, plural) {
        (0, _) => "",
        (1, _) => " mil ",
        (2, false) => " milhão ",
        (2, true) => " milhões ",
        (3, false) => " bilhão ",
        (3, true) => " bilhões ",
        (4, false) => " trilhão ",
        (4, true) => " trilhões ",
        (5, false) => " quatrilhão ",
        (5, true) => " quatrilhões ",
        (6, false) => " quintilhão ",
        (6, true) => " quintilhões ",
        _ => panic!("Valor inválido"),
    }
}

/// Retorna a leitura de um número entre 0 e 999
fn ler_classe(numero: u16) -> String {
    match numero {
        0 => "zero".to_owned(),
        1 => "um".to_owned(),
        2 => "dois".to_owned(),
        3 => "três".to_owned(),
        4 => "quatro".to_owned(),
        5 => "cinco".to_owned(),
        6 => "seis".to_owned(),
        7 => "sete".to_owned(),
        8 => "oito".to_owned(),
        9 => "nove".to_owned(),
        10 => "dez".to_owned(),
        11 => "onze".to_owned(),
        12 => "doze".to_owned(),
        13 => "treze".to_owned(),
        14 => "catorze".to_owned(),
        15 => "quinze".to_owned(),
        16 => "dezesseis".to_owned(),
        17 => "dezessete".to_owned(),
        18 => "dezoito".to_owned(),
        19 => "dezenove".to_owned(),
        20 => "vinte".to_owned(),
        30 => "trinta".to_owned(),
        40 => "quarenta".to_owned(),
        50 => "cinquenta".to_owned(),
        60 => "sessenta".to_owned(),
        70 => "setenta".to_owned(),
        80 => "oitenta".to_owned(),
        90 => "noventa".to_owned(),
        100 => "cem".to_owned(),
        200 => "duzentos".to_owned(),
        300 => "trezentos".to_owned(),
        400 => "quatrocentos".to_owned(),
        500 => "quinhentos".to_owned(),
        600 => "seicentos".to_owned(),
        700 => "setecentos".to_owned(),
        800 => "oitocentos".to_owned(),
        900 => "novecentos".to_owned(),
        21..=29 => "vinte e ".to_owned() + &ler_classe(numero - 20),
        31..=39 => "trinta e ".to_owned() + &ler_classe(numero - 30),
        41..=49 => "quarenta e ".to_owned() + &ler_classe(numero - 40),
        51..=59 => "cinquenta e ".to_owned() + &ler_classe(numero - 50),
        61..=69 => "sessenta e ".to_owned() + &ler_classe(numero - 60),
        71..=79 => "setenta e ".to_owned() + &ler_classe(numero - 70),
        81..=89 => "oitenta e ".to_owned() + &ler_classe(numero - 80),
        91..=99 => "noventa e ".to_owned() + &ler_classe(numero - 90),
        100..=199 => "cento e ".to_owned() + &ler_classe(numero - 100),
        200..=299 => "duzentos e ".to_owned() + &ler_classe(numero - 200),
        300..=399 => "trezentos e ".to_owned() + &ler_classe(numero - 300),
        400..=499 => "quatrocentos e ".to_owned() + &ler_classe(numero - 400),
        500..=599 => "quinhentos e ".to_owned() + &ler_classe(numero - 500),
        600..=699 => "seiscentos e ".to_owned() + &ler_classe(numero - 600),
        700..=799 => "setecentos e ".to_owned() + &ler_classe(numero - 700),
        800..=899 => "oitocentos e ".to_owned() + &ler_classe(numero - 800),
        900..=999 => "novecentos e ".to_owned() + &ler_classe(numero - 900),
        _ => panic!("Valor inválido"),
    }
}

/// Extrai a leitura das classes sem especificar os sufixos,
/// retornando um vetor contendo as classes, da maior para a menor.
fn separar_classes(numero: u128) -> Vec<u16> {
    let mut resposta = vec![];
    let mut n = numero;

    if n == 0 {
        resposta.push(0);
    }

    while n > 0 {
        resposta.push((n % 1000) as u16);

        n /= 1000;
    }

    resposta.reverse();
    resposta
}

#[cfg(test)]
mod tests {
    #[test]
    fn separar_classes() {
        let classes = super::separar_classes(12_387_459);
        assert_eq!(classes, vec![12, 387, 459]);
    }

    #[test]
    fn ler_classe() {
        assert_eq!(super::ler_classe(545), "quinhentos e quarenta e cinco");
        assert_eq!(super::ler_classe(0), "zero");
        assert_eq!(super::ler_classe(3), "três");
        assert_eq!(super::ler_classe(14), "catorze");
    }

    #[test]
    fn sufixo_classe() {
        let milhoes = super::sufixo_classe(2, true);
        let bilhao = super::sufixo_classe(3, false);
        assert_eq!(milhoes, " milhões ");
        assert_eq!(bilhao, " bilhão ");
    }
}
