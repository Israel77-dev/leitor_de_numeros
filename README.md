# Leitor de números em português

Pequeno programa escrito em Rust que recebe um número e retorna sua leitura por extenso.
Suporta qualquer número no intervalo de 0 (zero) a 999 999 999 999 999 999 999 (novecentos e noventa e nove quintilhões novecentos e noventa e nove quatrilhões novecentos e noventa e nove trilhões novecentos e noventa e nove bilhões novecentos e noventa e nove milhões novecentos e noventa e nove mil novecentos e noventa e nove).

# Compilação e uso

Esse programa foi desenvolvido para funcionar em sistemas operacionais tipo Unix. Atualmente só foi testado em sistemas baseados em GNU/Linux, porém deve funcionar em sistemas BSD/macOS sem problemas.

## Instalando rust

Esse programa foi escrito em Rust, então antes de qualquer coisa é preciso instalar um compilador para esta linguagem.

A forma mais fácil de fazer isso é através do [programa Rustup](https://www.rust-lang.org/pt-BR/tools/install), que pode baixar e instalar o compilador de Rust, o conjunto de ferramentas e gerenciador de pacotes Cargo, bem como gerenciar as atualizações de ambos. O programa pode ser instalado através de um comando de terminal ou através de [outros métodos](https://forge.rust-lang.org/infra/other-installation-methods.html).

## Construindo o arquivo

Primeiro clone este repositório do github para um diretório local:

```shell
$ git clone https://github.com/Israel77-dev/leitor_de_numeros.git
```

Acesse o diretório e faça a compilação do programa através da ferramenta cargo:

```shell
$ cd leitor_de_números
$ cargo build --release
```

Depois disso um arquivo binário com o nome leitor_de_numeros deve aparecer dentro do diretório target/release. Você pode "instalar" o programa copiando esse arquivo para algum diretório que faça parte da variável $PATH.

## Utilizando o programa

Para executar o programa dentro do diretório do projeto, basta utilizar o comando cargo run seguido do(s) número(s) a ser(em) lido(s). Caso mais de um número seja passado como argumento o programa exibirá a leitura de cada um deles em uma linha separada, seguindo a ordem em que foram passados. Exemplo:

```shell
$ cargo run 12375
doze mil trezentos e setenta e cinco
$ cargo run 154 78669
cento e cinquenta e quatro
setenta e oito mil seiscentos e sessenta e nove
```

Alternativamente pode ser executado o binário gerado na etapa de compilação:

```shell
$ cd target/release
$ ./leitor_de_numeros 198
cento e noventa e oito
```
Caso o programa tenha sido "instalado" (adicionado ao $PATH), pode se executá-lo em qualquer diretório como um comando:
```shell
$ leitor_de_numeros 1699
mil seiscentos e noventa e nove
```