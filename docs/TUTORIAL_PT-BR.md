> Este tutorial irá descrever o processo de desenvolvimento desta CLI.
> A ideia é ter uma documentação que sirva de exemplo para estudos futuros.

---

- [Definições técnicas](#definições-técnicas)
- [Etapas de desenvolvimento](#etapas-de-desenvolvimento)
  - [Task 1: Inicializando CLI com o comando help e version](#task-1-inicializando-cli-com-o-comando-help-e-version)
    - [TDLR;](#tdlr)
    - [Objetivo:](#objetivo)
    - [Vamos ao código](#vamos-ao-código)
    - [Epilogo](#epilogo)

---

# DESENVOLVENDO quati-cli

Quati-cli é um projeto criado no intuito de otimizar o processo de escrita de mensagens de commit.
Através de um simples comando será possível salvar todas as alterações e subir para a origem com uma mensagem descritiva sobre as modifições naquele commit.

Utilizando de agentes de IA, como copilot, será solicitada a IA para gerar a mensagem de commit.
Para isso, iremos passar um arquivo diff, com as alterações, e o que mais achar necessário.

Quati-cli consistem em 3 comandos: `start`, `save`, `update`.

- `start`: Cria uma nova branch na origem
- `save`: Efetua um commit local com a mensagem gerada pela IA
- `update`: Além de agir como o save, também faz push na branch atual.

## Definições técnicas

- Linguage: [Rust](https://rust-lang.org/)
- Principais depências:
  - Contrução da CLI: [clap](https://docs.rs/clap/latest/clap/_concepts/index.html)
- Metodologia de desenvolvimento:
  - Test Driven Development ([TDD](https://pt.wikipedia.org/wiki/Test-driven_development))

## Etapas de desenvolvimento

Nesta sessão será descrita em formado de Tasks, o processo de construção da CLI.

### Task 1: Inicializando CLI com o comando help e version

#### TDLR;

Está com pressa? Esse é o código implementado para a task. A explicação está logo após.

```rs
// tests/args.rs
use assert_cmd::cargo::*; // Import cargo_bin_cmd! macro and methods
use predicates::prelude::*;

#[test]
fn should_show_help() {
    let mut cmd = cargo_bin_cmd!("quati");
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "CLI to manage git changes with AI assistance",
    ));
}
```

```rs
// src/args.rs
use clap::Parser;

/// CLI to manage git changes with AI assistance
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {}
```

```rs
mod args;

use args::Args;
use clap::Parser;

fn main() {
    Args::parse();
}
```

#### Objetivo:

- Executando o comando `quati` ou `quati -h` ou `quati --help`  e exibir o helper da CLI
- Executando o comando `quati -v` ou `quati --version` irá exibir a versão atual da aplicação. 

Toda CLI tem um comando `help` que explica como a utiliza.
Iniciar o desenvolvimento com o `help` nos auxilia na construção da estrutura da CLI sem que precisemos implementar muito código específico.
Apesar de nem todas possuirem o comando version, iremos implementar para seguirmos com um versionamento `semver` do projeto.

Seguindo a ideia do TDD, iremos primeiro criar o teste com o mínimo necessário.

Quando se pensa no mínimo, a primeira coisa que podemos imagina seria executar executar o comando e trazer o titulo da CLI.

Logo o primeiro caso de uso seria:

| Nº  | Dado que                                                                   | Quando                                  | Então                                                                                           |
| --- | -------------------------------------------------------------------------- | --------------------------------------- | ----------------------------------------------------------------------------------------------- |
| 1   | executo `quati -h` no terminal **OU** executo `quati --helper` no terminal | desejo ter informações de como usar CLI | exibo o mensage de `help` no projeto com o texto `CLI to manage git changes with AI assistance` |

 

> O caso de uso acima foi descrito utiilzando [GIVEN-WHEN-THEN](https://martinfowler.com/bliki/GivenWhenThen.html), em uma versão traduzida para PT-BR.
> Todos os casos de suso serão escritos desta maneira

#### Vamos ao código

Iremos implementar um teste de integração para este exemplo.
O principal motivo para isso é que o a função `help` é nativa da biblioteca `clap`.

Neste momento, queremos testar somente o titulo da CLI.

Por convensão, `cargo` interpreta o contexto de algumas pastas por padrão.

- `tests/`: testes de integração
- `benches/`: para benchmarks
- `examples/`: para exemplos
- `src/`: para código-fonte principal do projeto

> Referência: [Testing CLI applications by running them](https://rust-cli.github.io/book/tutorial/testing.html#testing-cli-applications-by-running-them)

Assim sendo, criemos o nosso arquivo de teste em `tests/args.rs`. Veja abaixo o código de teste

```rs
use assert_cmd::cargo::*;
use predicates::prelude::*;

#[test]
fn should_show_help() {
    let mut cmd = cargo_bin_cmd!("quati");
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "CLI to manage git changes with AI assistance",
    ));
}
```

Para este arquivo funcionar, você precisará adicionar duas dependências ao projeto.

```sh
cargo add assert_cmd predicates
```

- [assert_cmd](https://docs.rs/assert_cmd/latest/assert_cmd/): auxilia no processo para fazer testes de integrações.
- [precidate](https://docs.rs/predicates/latest/predicates/): auxilia na validação dos campos


Agora vamos criar o args.rs que servirá de base para os comandos da CLI:

```rs
use clap::Parser;

/// CLI to manage git changes with AI assistance
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {}
```

Para o funcionamento deste código será necessário instalar o pacote [clap](https://docs.rs/clap/latest/clap/).
Clap é um incrível pacote fará todo o processo relacionado aos argumentos da CLI.

Para este momento, nosso teste irá somente validar a descrição da CLI.

A descrição da CLI está no comentário de documentation externa ([Outer Docummentation Comment](https://doc.rust-lang.org/rust-by-example/meta/doc.html)) inicializado por `///`, que serve para documentar o escopoco a seguir. `clap` utiliza desta documentação para descrever a CLI.

Incializamos também o [Parser](https://cursos.alura.com.br/forum/topico-o-que-e-de-fato-parse-ou-parsing-71499) atravez da [syntax de atributos](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/attributes.html) ([from rust docs](https://doc.rust-lang.org/reference/attributes.html)) `#[]`  (que lembra um pouco annotatins em outras linguages `@AlgumaCoisa`).

O próximo atributo definido é o `command` que pode ser utilizado para definir coisas básicas da CLI. Isso também é usado pelo `clap`. No exemplo acima não definimos alguns valores, como version, que é coletado de `Cargo.toml` (veja abaixo) e about que pegará do nosso `///`.

```toml
[package]
name = "quati"
version = "0.1.0"
edition = "2024"
...
```

A struct está vazia, pois ainda não queremos definir opções e comandos para o nossa CLI neste momento.

> Struct é algo que irá definir uma estrutura, um tipo, um objeto. Em java, poderia dizer que seria similar a um record, em typescript um type. **Similar**, ou seja, não é a mesma coisa, mas é por ai.

Agora vamos atualizar nossa main:

```rs
mod args;

use args::Args;
use clap::Parser;

fn main() {
    Args::parse();
}
```

Utilizamos o arquivo args, logo precisamos por [mod args](https://doc.rust-lang.org/rust-by-example/mod.html) para indicar ao compilador que o arquivos args faz parte do módulo atual.

Com isso podemos importar `Args` usando `use args::Args`. Como também precisaremos usar a função parse do clap, importamos usando `use args::Parser`, e assim na função main, executamos `Args::parse()` que inicializará a commandline.

#### Epilogo

Este é o código da primeira tarefa. Nesta tarefa implementamos:

- Teste de integração para exibir o help (`args.rs`).
- Dependências de teste adicionadas (`assert_cmd`, `predicates`).
- Módulo de argumentos (`args.rs`) com `#[derive(Parser)]` e doc comment como descrição.
- Metadados do binário (version) ligados ao Cargo.toml via #[command(version, ...)].
- Declaração mod args; e chamada Args::parse() em main.rs.
- Estrutura mínima pub struct Args {} sem opções ainda.

Conceitos vistos nesta tarefa:

- Testes de integração (pasta `tests`) e execução de binários de CLI para verificar comportamento.
- Uso das crates de teste: `assert_cmd` e `predicates` para invocar o binário e validar saída.
- Sistema de módulos: declaração `mod args;` e organização de código em arquivos separados (args.rs).
- Importações com use (ex.: `use args::Args;`, `use clap::Parser;`).
- Definição de struct (`pub struct Args {}`) como tipo que representa a CLI.
- Derive de traits/implementações via atributos (`#[derive(Parser)]`).
- Atributos de comando do clap (`#[command(version, about, long_about = None)]`) para configurar metadados da CLI.
- Comentários de documentação externa (`///`) usados como descrição da CLI e aproveitados pelo clap.
- Chamada do parser gerado pelo clap (`Args::parse()`) para inicializar a leitura de argumentos.
- Dependência em metadados do pacote (`Cargo.toml`) para obter a versão do binário.
- Convenções do Cargo sobre layout de projeto (`src`, `tests`, `examples/`, `benches/`).
- Princípio de Test Driven Development (TDD) aplicado ao desenvolvimento da CLI.
