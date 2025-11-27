> Este tutorial irá descrever o processo de desenvolvimento desta CLI.
> A ideia é ter uma documentação que sirva de exemplo para estudos futuros.

---

- [DESENVOLVENDO quati-cli](#desenvolvendo-quati-cli)
  - [Definições técnicas](#definições-técnicas)
  - [Etapas de desenvolvimento](#etapas-de-desenvolvimento)
    - [Task 1: Inicializando CLI com o comando help e version](#task-1-inicializando-cli-com-o-comando-help-e-version)
      - [TDLR;](#tdlr)
      - [Objetivo:](#objetivo)
      - [Vamos ao código](#vamos-ao-código)

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
Clap é incrível pacote fará todo o processo relacionado aos argumentos da CLI.

Para este momento, nosso teste irá somente validar a descrição da CLI. A descrição definida 
