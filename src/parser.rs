// Gramática dos números em português, para leitura dos números entre 0 e 999 999 999 999 999 999 999 (classe dos quintilhões):
// número ::= [classe] [sufixo-de-classe];
// classe ::= "zero" | [dez-a-dezenove] | ([centena] "e" [dezena] "e" unidade) ;
// sufixo-de-classe ::= "mil" | "milhão" | "milhões" | "bilhão" | "bilhões" | "trilhão" | "trilhões" | "quatrilhão" | "quatrilhões" | "quintilhão" | "quintilhões";
// centena ::= "cem" | "duzentos" | "trezentos" | "quatrocentos" | "quinhentos" | "seicentos" | "setecentos" | "oitocentos" | "novecentos";
// dezena ::= "vinte" | "trinta" | "quarenta" | "cinquenta" | "sessenta" | "setenta" | "oitenta" | "noventa";
// dez-a-dezenove ::= "dez" | "onze" | "doze" | "treze" | "catorze" | "quinze" | "dezesseis" | "dezessete" | "dezoito" | "dezenove";
// unidade ::=  "um" | "dois" | "três" | "quatro" | "cinco" | "seis" | "sete" | "oito" | "nove";

// 3 -> três
// 18 -> dezoito
// 97 -> noventa e sete
// 946 -> novecentos e quarenta e seis
// 1000 -> mil
// 1 285 -> mil duzentos e oitenta e cinco
// 15 987 -> quinze mil novecentos e oitenta e sete
// 446 744 073 709 551 616 -> Quatrocentos e quarenta e seis quatrilhões setecentos e quarenta e quatro trilhões setenta e três bilhões setecentos e nove milhões quinhentos e cinquenta e um mil seicentos e dezesseis

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

pub fn ler_numero(numero: u128) -> String {
    let classes = extrair_classes(numero);
    let mut leitura = String::new();
    let num_classes = classes.len();
    let mut id_classe = (classes.len() - 1) as u8;

    for classe in classes {
        leitura.push_str(&match (classe, id_classe, num_classes) {
            // Número zero
            (0, 0, 1) => "zero".to_owned(),
            // Classe nula não é lida
            (0, _, _) => "".to_owned(),
            // 1000, é lido apenas como mil e não um mil
            (1, 1, _) => "mil ".to_owned(),
            // Classe das unidades simples (sem sufixo)
            (c, 0, 1) => ler_classe(c),
            // Classe das unidades simples com valor menor que 100 é prefixada com o conectivo "e"
            (c, 0, _) if c < 100 => format!("e {}", ler_classe(c)),
            // Caso singular (exceto mil)
            (1, i, _) => format!("um{}", sufixo_classe(i, false)),
            // Demais casos (plural)
            (c, i, _) => format!("{}{}", ler_classe(c), sufixo_classe(i, true)),
        });

        if id_classe > 0 {
            id_classe -= 1;
        }
    }

    leitura.trim_end().to_owned()
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
/// retornando-as da maior para a menor
fn extrair_classes(numero: u128) -> Vec<u16> {
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
mod test {
    use super::*;

    #[test]
    fn lista_classes() {
        assert_eq!(extrair_classes(0), vec![0]);
        assert_eq!(extrair_classes(3), vec![3]);
        assert_eq!(extrair_classes(145), vec![145]);
        assert_eq!(extrair_classes(15_692), vec![15, 692]);
        assert_eq!(extrair_classes(1_935_688), vec![1, 935, 688]);
    }

    #[test]
    fn leitura_de_numeros() {
        assert_eq!(ler_numero(0), "zero");
        assert_eq!(ler_numero(7), "sete");
        assert_eq!(ler_numero(1232), "mil duzentos e trinta e dois");
        assert_eq!(ler_numero(10375), "dez mil trezentos e setenta e cinco");
        assert_eq!(
            ler_numero(2693412),
            "dois milhões seiscentos e noventa e três mil quatrocentos e doze"
        );
        assert_eq!(ler_numero(1035), "mil e trinta e cinco");
        assert_eq!(ler_numero(1_000_035), "um milhão e trinta e cinco");
        assert_eq!(
            ler_numero(3_465_000),
            "três milhões quatrocentos e sessenta e cinco mil"
        );
    }
}
