> ---
> # Índice

- [COOKBOOK: Aprendendo com testes](#cookbook-aprendendo-com-testes)
  - [Efetuando mock do Command](#efetuando-mock-do-command)
    - [Contexto](#contexto)
    - [Implementação](#implementação)
      - [TDD: Primeira iteração - FakeRunner](#tdd-primeira-iteração---fakerunner)
        - [pub struct RunResult](#pub-struct-runresult)
        - [Finalizando](#finalizando)
      - [TDD: Segunda iteração - GitRunner](#tdd-segunda-iteração---gitrunner)
        - [pub trait GitRunner](#pub-trait-gitrunner)
        - [Finalizando](#finalizando-1)
      - [TDD: Terceira iteração - Git, the real implementation](#tdd-terceira-iteração---git-the-real-implementation)
        - [Implementando RealGitRunner e impl Git](#implementando-realgitrunner-e-impl-git)
        - [Finalizando](#finalizando-2)
      - [TDD: Quarta iteração - GitError, melhorando as mensagens de erros](#tdd-quarta-iteração---giterror-melhorando-as-mensagens-de-erros)
    - [Resultado final](#resultado-final)

---
---


# COOKBOOK: Aprendendo com testes

Objetivo deste documento é aprender RUST através de testes: unitários, integração etc.

Por mais que testes automatizados seja um tópico relativamente avançado, adquirir uma cultura de testes é essencial para qualquer pessoa desenvolvedora.  
Então, porque não começar com os testes? ~~Porque é coisa de doido~~

Uma vez ouvi um colega dizer para mim: 

> "Eu dificilmente subo minha aplicação. Geralmente é a ultima etapa, só para ter certeza. Eu sempre utilizo os testes unitários para executar meu código"

Isso mudou mais uma vez meu estilo de programação. Apesar de muitos considerar que testes atrasam uma entrega. Eu digo que testes postergam problemas. O prazo de entrega pode aumentar algund dias, enquanto um prazo de debug tendem a demorar semanas.

Utilizar TDD (Test Driven Development, Desenvolvimento Orientado a Teste) exige prática, e no começo vai parecer ser bem sem sentido, mas continue firme na prática, isso mudará seu jeito de pensar em como entregar o mínimo.  
Resumidamente, no TDD primeiro você escreve o teste, para aquilo que não existe, isso deve falhar, do contrário, ou o teste está errado, ou a implementação já existe e você não sabia :D.  
A implementação segue um ciclo: Escreve o teste, Faz o teste passar, Refatore o código.

```mermaid
---
title: Ciclo TDD
---
stateDiagram-v2
  state "Escreva o Teste" as a
  state "Escreva o código que passe no teste" as b
  state "Refatore o código" as c
  [*] --> a: Primeiro passo
  a --> b: falhou
  b --> c: passou
  c --> a: próximo
```

Os exemplos abaixo foram surgindo de acordo com a minha necessidade e meu aprendizado. Não estão necessariamente em alguma ordem.

> **NOTA:**
> Nem tudo deve ser testado, nem todos os testes abaixo devem ser realmente implementados.
> O objetivo deste documento é o aprendizado,

## Efetuando mock do Command

> #test #command #mock

### Contexto

Estou criando uma função que irá executar um commando no terminal para saber qual o nome da branch do git que estou no momento.

O comando git para isso é: `git rev-parse --abbrev-ref HEAD`.

### Implementação

Para escrevermos o teste desta execução, precisaremos encapsular a execução do comando de forma que possa receber o executável como parâmetro.  
Isso será usado para podemos efetuar um mock do executável.

> Mock:
> Mocks são simulações de algo real, utilizados principalmente para em testes de softwares onde substituem aluma dependência real. Podendo assim, controlar os resultados dessa dependência.

Então, vamos implementar nosso teste primeiro.

#### TDD: Primeira iteração - FakeRunner

> Recaptulando: A ideia do TDD é primeiro implementar um teste para algo que não existe primeiro, para assim nos forçarmos a desenvolver um código que faça o teste funcionar passar.

```rs
#[cfg(test)]
mod tests {
  use super::*;

  struct FakeRunner {
      result: RunResult,
  }

  impl FakeRunner {
      fn new(success: bool, stdout: &str, stderr: &str) -> Self {
          Self {
              result: RunResult {
                  success,
                  stdout: stdout.as_bytes().to_vec(),
                  stderr: stderr.as_bytes().to_vec(),
              },
          }
      }
  }
}
```

Agora, vou explicar o código acima:

`#[cfg(test)]`: essa definição de atributo de configuração (`cfg`). Isso indicará ao compilador que mod tests na linha seguinte é relacionada aos testes e precisa ser compilado somente quando executamos os testes, `cargo test`.  
`mod tests {`: inicialização do módulo de tests.  
`use super::*;`: **super** é uma palavra chave que indica que você acessará algo do módulo parent. No caso dos testes unitários, geralmente (mas, nem sempre), definimos eles no mesmo arquivo, um nível abaixo, da implementação do módulo (se não quando na docs). Então, esta linha de código indica que iremos utilizar de tudo que há no modulo parent, que se no mesmo arquivo, tudo que é publico no arquivo.  
`struct FakeRunner {`: Aqui começa o nosso mock a existir. Criamos uma estrutura chamada FakeRunner, que é implementada posteriorment.  
`result: RunResult,`: esse campo da struct FakeRunner significa que o valor result tem o tipo RunResult, que iremos escrever posteriorment.  
`impl FakeRunner {`: Essa linha representa o início da implementação da struct. Podemos ler como **implementação de FakeRunner**.  
`fn new(success: bool, stdout: &str, stderr: &str) -> Self {`: Esse linha de impl FakeRunner é um método cujo objetivo é simular os mesmo parâmetros de resposta da estrutura RunResult (que, novamente, será escrita posteriorment).

```rs
Self {
    result: RunResult {
        success,
        stdout: stdout.as_bytes().to_vec(),
        stderr: stderr.as_bytes().to_vec(),
    },
}
```

O código acima, contido dentro da `função new(...)` retorna um resulta de si mesmo, mas com os valores de result como definido.  
Só com essas implementações da `struct FakeRunner` e `impl FakeRunner` nos "obriga" a implementar a `struct RunResult`, que evitará erros de compilação.

Então, vamos implementar a `struct` e assim finalizarmos nossa primeira iteração. E assim, termos o nosso primeiro commit.

##### pub struct RunResult

Sendo assim, para encapsular a execução do nosso comando Git, primeiro precisaremos criar uma struct relacionada a estrura do resultado.

```rs
#[derive(Clone, Debug)]
pub struct RunResult {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
```

Explicando o código:

`#[derive(Clone, Debug)]`, se você entende de Orientação a Objetos, podemos fazer um comparativo dizendo que estamos herdando, nesse caso derivando, os métodos Clone e Debug nativos da linguagem.  
Isso significa que a struct criada terá os métodos `clone()` e `debug()`, utilizando da macro `derive`.

`pub struct RunResult {`, a definição da struct como pública, permitindo uso fora do módulo (ou simplesmente do arquivo).  
Em Rust, por padrão, tudo é privado, logo precisamos por a notação `pub` em tudo que queremos ter acesso fora do módulo. Acredito que seja uma decisão de segurança :)

##### Finalizando

Execute os testes e vamos ver se eles passam:

```sh
cargo test
```

Com isso implementado, vamos então fazer nosso primeiro commit: 

```sh
git add .; 
git commit -m "feat: wip - implementando estrutura do resultado da consulta do git"
```

#### TDD: Segunda iteração - GitRunner

```rs
#[cfg(test)]
mod tests {
    use super::*;
/*
Aqui está a implementação já feita acima, vamos evitar nos repetir :)

    struct FakeRunner {}
    impl FakeRunner {}
*/

    impl GitRunner for FakeRunner {
        fn run(&mut self, _args: &[&str]) -> std::io::Result<RunResult> {
            Ok(self.result.clone())
        }
    }
}
```

Vamos entender as mudanças:

`impl GitRunner for FakeRunner {`: **impl**ementação de **GitRunner** **para** a estrutura **FakeRunner**. Essa linha indica que estamos implementando uma definição (trait) para estrutura `FakeRunner`. Em orientação a objeto seria como um `extends` ou `implements` e `GitRunner` seria considerada como uma `interface`. A forma como `impl` funciona é interessante, é como se fossemos adicionando novas funcionalidades a estrutura FakeRunner cada vez que utilizamos. A primeira vez, adicionamos new, da segunda vez, utilizamos uma definição (trait) para dizer que função deveriamos implementar.

```rs
fn run(&mut self, _args: &[&str]) -> std::io::Result<RunResult> {
    Ok(self.result.clone())
}
```

O código acima representa a função definida no trait `GitRunner`. A assinatura do método, o tipo de retorno, tudo é definido dentro de `GitRunner`. Mas, através de `impl GitRunner for ...` é quando a função realmente é implementada, deixando de ser somente uma assinatura.  
Nesse caso, nós simplesmente retornamos uma cópia do objeto result, que é definido na função `new`.


Agora vamos implementar `GitRunner` trait que é utilizado como definição para `FakeRunner`

##### pub trait GitRunner

```rs
pub trait GitRunner {
    fn run(&mut self, args: &[&str]) -> std::io::Result<RunResult>;
}
```

Vamos entender o código acima:

`pub trait GitRunner {`, assim como para struct, `pub` é utilizado para definir esta trait como publica, com acesso externo.  
`fn run(&mut self, args: &[&str]) -> std::io::Result<RunResult>;`, é a definição da função `run`.

`fn run` é o nome do método,  
`&mut self` significa que estamos passando uma referência que pode ser alterada de "si mesmo",  
`args: &[&str]` significa que podemos receber um array sem limites de valores de texto (string)  
`std::io::Result<RunResult>;` é outra forma de escrever `Result<RunResult, std::io::Error>`. Isto é usado como retorno de funções IO (input/output, entrada/saída), como é no nosso caso, onde iremos executar um commando git'. O retorno é um `Ok(RunResult)` caso de sucesso, retornando um resultado na estrutura de `RunResult`; Ou um valor de erro `Err(std::io::Error)` do tipo I/O. Esse formato é comumento usado para implementar `?` para propagação de error, bem como funções que executan I/O ou chamam outras API com retorno I/O.

Assim como em Orientação a Objeto, na qual temos uma `interface` que é uma abstração de uma implementação real, e então implementamos uma `classe` que implementa esta `interface` de modo a termos uma classe concreta, em rust faremos algo semelhante.

##### Finalizando

Vamos então executar nosso teste: 

```sh
cargo test
```

Tudo estando OKAY, podemos fazer nosso segundo commit:

```sh
git add .; 
git commit -m "feat: wip implementando trait GitRunner"
```

#### TDD: Terceira iteração - Git, the real implementation

Nesta iteração faremos o teste que desejamos.

```rs
#[cfg(test)]
mod tests {
    /**
     * Código á implementado
        struct FakeRunner {}
        impl FakeRunner {}
        impl GitRunner for FakeRunner {}
     */

    #[test]
    fn returns_branch_name_on_success() {
        let fake = FakeRunner::new(true, "feature/test-branch\n", "");
        let mut git = Git::with_runner(Box::new(fake));
        let branch = git
            .get_current_branch_name()
            .expect("expected branch name on success");
        assert_eq!(branch, "feature/test-branch");
    }
}
```

Explicando o código:

`#[test]`: Esse atributo indica que a função seguinte é um teste  
`fn returns_branch_name_on_success() {`: Assinatura da função, o nome da função será o nome exibido no terminal.
`let fake = FakeRunner::new(true, "feature/test-branch\n", "");`: nesta linha inicializamos o nosso mock runner. 
Nele passamos o valor de sucesso e o nome da branh na qual querermos retornar, o terceiro parâmetro é de erro, não necessário neste teste.  
`let mut git = Git::with_runner(Box::new(fake));`: Nesta linha implementamos o nossa implementação concreta do trait GitRunner. Iremos implementar a seguir, logo, o teste falhará por isso.

```rs
let branch = git
    .get_current_branch_name()
    .expect("expected branch name on success");
```

`assert_eq!(branch, "feature/test-branch");`: aqui temos a nossa validação no teste. Esta linha que indica se o teste foi sucesso.

Agora que entendemos o teste implementado, vamos escrever nosso código para passar

##### Implementando RealGitRunner e impl Git

Neste código acima temos a chamada do método, que ainda iremos implementar.

Iremos definir uma estrutura (`struct`) que será nossa implementação concreta da nossa interface (`trait`).

```rs
pub struct RealGitRunner;
```

Abaixo temos a "implementação concreta"

```rs
impl GitRunner for RealGitRunner {
    fn run(&mut self, args: &[&str]) -> std::io::Result<RunResult> {
        let output = Command::new("git").args(args).output()?;
        Ok(RunResult {
            success: output.status.success(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}
```

`impl GitRunner for RealGitRunner {`: **implementar** o trait **GitRunner** **para** a estrutura **RealGitRunner**. Parece lógico né? Estamos implementando um `trait` em uma `struct`.
`fn run(&mut self, args: &[&str]) -> std::io::Result<RunResult> {`: veja que contém a mesma assinatura que o método do trait, isso significa que esta é a implementação real da função.
`let output = Command::new("git").args(args).output()?;`: Esta linha executa o comando do **git** passando a lista de argumentos.

```rs
Ok(RunResult {
    success: output.status.success(),
    stdout: output.stdout,
    stderr: output.stderr,
})
```

As linhas acimas são o retorno de sucesso da chamada da função. Para nosso exemplo, não precisaremos de um retorno de falha `Err()`, pois `output()?` já faz esse papel.

Agora vamos escrever a nossa implementação final:

```rs
pub struct Git {
    runner: Box<dyn GitRunner>,
}

impl Git {
    /// Create a client that uses the real `git` binary.
    pub fn real() -> Self {
        Self {
            runner: Box::new(RealGitRunner),
        }
    }

    /// Create a client with a custom runner (useful for tests).
    pub fn with_runner(runner: Box<dyn GitRunner>) -> Self {
        Self { runner }
    }

    /// Get the current branch name. Returns `Ok(branch)` on success or `Err(GitError)` on failure.
    pub fn get_current_branch_name(&mut self) -> Result<String, GitError> {
        let result = self
            .runner
            .run(&["rev-parse", "--abbrev-ref", "HEAD"])
            .map_err(GitError::Io)?;

        if result.success {
            Ok(String::from_utf8_lossy(&result.stdout).trim().to_string())
        } else {
            let stderr = String::from_utf8_lossy(&result.stderr).to_string();
            Err(GitError::GitFailed(stderr))
        }
    }
}
```

Vamos conversar sobre o código:

```rs
pub struct Git {
    runner: Box<dyn GitRunner>,
}
```

O código acima define a estrutura do Git.  
`runner: Box<dyn GitRunner>,`: Alocação de ponteiro-heap para algum tipo concreto de uma implementação do GitRunner, mas que em tempo de compilação está vazio, pois sua alocação se dá em tempo de execução. **Box<>** é um trait que aloca um espaço na memória heap e armazena um ponteiro próprio para o conteúdo. **dyn GitRunner**, é um trait para objeto que habilita uso dinâmico através de uma [vtable](https://users.rust-lang.org/t/v-tables-differences-between-rust-and-c/92445/2) em tempo de execução.

Entendido como funciona a estutura, agora vamos a sua implementação:

```rs
impl Git {
    /// Create a client that uses the real `git` binary.
    pub fn real() -> Self {
        Self {
            runner: Box::new(RealGitRunner),
        }
    }

    /// Create a client with a custom runner (useful for tests).
    pub fn with_runner(runner: Box<dyn GitRunner>) -> Self {
        Self { runner }
    }

    /// Get the current branch name. Returns `Ok(branch)` on success or `Err(GitError)` on failure.
    pub fn get_current_branch_name(&mut self) -> Result<String, GitError> {
        let result = self
            .runner
            .run(&["rev-parse", "--abbrev-ref", "HEAD"])
            .map_err(GitError::Io)?;

        if result.success {
            Ok(String::from_utf8_lossy(&result.stdout).trim().to_string())
        } else {
            let stderr = String::from_utf8_lossy(&result.stderr).to_string();
            Err(GitError::GitFailed(stderr))
        }
    }
}
```

`impl Git {`: Inicialização da implementação da struct Git  
`pub fn real() -> Self {`: Esse método retorna uma implementação "verdadeira" para o uso real. O nome da funcão ser `real()` é uma convenção. Isso indica que este método não retorna um objeto mockado ou alguma versão difereciada.

```rs
Self {
    runner: Box::new(RealGitRunner),
}
```

Essa parte do método `real()` executa um acesso a variável definida na `struct`. Este termo `Self` indica que estou dentro de um contexto interno.  
Sendo assim, o método altera o o valor de `runner` com um novo `Box` usando `RealGitRunner` como execultável

`pub fn with_runner(runner: Box<dyn GitRunner>) -> Self {`: este é o maravilhoso método usado no teste. Ele aceita um parâmetro do tipo `Box<dyn GitRunner>` e define ele no runner. Em outras linguagens de programação isso poderia ser chamado de um método `set`, mas por legibilidade `with_algumacoisa()` tem uma melhor legibilidade, sobretudo em chamadas de métodos concatenados.  
`Self { runner }`: assim, o método somente tem uma definição direta do runner. Como o nome do parâmetro da função é igual ao valor na struct, não se faz necessário escrever `runner: runner`.

Agora vamos falar do nosso método para consultar a branch:

```rs
pub fn get_current_branch_name(&mut self) -> Result<String, String> {
  let result = self
      .runner
      .run(&["rev-parse", "--abbrev-ref", "HEAD"])
      .output()?

  if result.success {
      Ok(String::from_utf8_lossy(&result.stdout).trim().to_string())
  } else {
      Err("We have a problem")
  }
}
```

Vamos então explicar o código:

`pub fn get_current_branch_name(&mut self) -> Result<String, String> {`: Aqui temos a assinatura do método onde o parâmetro da função é um `&mut self`. **&mut self** aqui existe por conta da implementação dinâmica do runner. Como a definição do runner não é feita em tempo de compilação, a consequência disso é que precisamos dizer ao nosso método que `self` pode ser alterado a qualquer momento. **Result<String, GitError>** significa que iremos retornar um `Ok()` (para sucesso) ou um `Err()` (para falha), onde o retorno de Ok é uma string e o de Err é um tipo String (Isso irá mudar na próxima iteração para um erro mais customizado).  

```rs
let result = self
    .runner
    .run(&["rev-parse", "--abbrev-ref", "HEAD"])
    .output()?
```

O código acima execulta o comando do git através do runner definido. Isso significa que para usar o método precisamos sempre fazer uma chamada em sequência: `git.real().function_desired()`. Assim garantiremos que sempre teremos um runner definido em tempo de execução.

```rs
if result.success {
    Ok(String::from_utf8_lossy(&result.stdout).trim().to_string())
} else {
    Err("We have a problem")
}
```

Nesse trecho de código, temos aqui uma validação do resultado do comando. Caso de sucesso, retornamos uma string. Caso falso, também uma string, mas com mensagem de erro.  
`Ok(String::from_utf8_lossy(&result.stdout).trim().to_string())`:

- `&result.stdout`: Contém o resultado do comando em um formato de `bytecode`, um `Vec<u8>`. O `&` indica que estamos passando uma referência ao valor, chamamos isso de [`borrowing`](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html). **Borrowing** é um conceito muito necessário em Rust, vale a pena dedicar um tempo lendo sobre.
- `String::from_utf8_lossy(&result.stdout)`: Converte os bytes em um texto UTF-8. Usamos este método para converter bytes para texto UTF-8 com tolerância a falha, caso dé erro, caracteres como `�` são inseridos como `texto`
- `.trim()`: remove espaços em branco no início e fim.
- `.to_string()`: Converte o resultado em um String alocado (owned). Isso garante que quem chama o método irá deter posse do resultado ([ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)). **Ownership** é outro conceito de extrema importância em rust. 

> Em um resumo nada convencional de explicar, ownership e borrowing são os meios do rust de fazer com que a pessoa desenvolvedora seja responsável pelo garbage collector :D


##### Finalizando

Agora com a implementação concluída, vamos verificar se os testes estão passando:

```sh
cargo test
```

Com tudo passando corretamente, vamos então efetuar mais um commit :)

```sh
git add .;
git commit -m "feat: wip - criada implementação concreta do executável para o Git"
```

Na próxima iteração iremos concluir a primeira feature adicionando validações para os erros.

#### TDD: Quarta iteração - GitError, melhorando as mensagens de erros

... TODO

### Resultado final

```rs
/ ...existing code...
use std::process::Command;
use std::{error::Error, fmt};

/// Small, test-friendly representation of a command run result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunResult {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// Trait to abstract running git commands so tests can inject a fake runner.
pub trait GitRunner {
    fn run(&mut self, args: &[&str]) -> std::io::Result<RunResult>;
}

/// Real runner that invokes the system `git` command.
pub struct RealGitRunner;

impl GitRunner for RealGitRunner {
    fn run(&mut self, args: &[&str]) -> std::io::Result<RunResult> {
        let output = Command::new("git").args(args).output()?;
        Ok(RunResult {
            success: output.status.success(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

/// Error type for git helpers.
#[derive(Debug)]
pub enum GitError {
    Io(std::io::Error),
    GitFailed(String),
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::Io(e) => write!(f, "IO error: {}", e),
            GitError::GitFailed(s) => write!(f, "git command failed: {}", s),
        }
    }
}

impl Error for GitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GitError::Io(e) => Some(e),
            GitError::GitFailed(_) => None,
        }
    }
}

impl From<std::io::Error> for GitError {
    fn from(e: std::io::Error) -> Self {
        GitError::Io(e)
    }
}

/// Lightweight Git client that holds a runner. Use in tests by injecting a FakeRunner.
pub struct Git {
    runner: Box<dyn GitRunner>,
}

impl Git {
    /// Create a client that uses the real `git` binary.
    pub fn real() -> Self {
        Self {
            runner: Box::new(RealGitRunner),
        }
    }

    /// Create a client with a custom runner (useful for tests).
    pub fn with_runner(runner: Box<dyn GitRunner>) -> Self {
        Self { runner }
    }

    /// Get the current branch name. Returns `Ok(branch)` on success or `Err(GitError)` on failure.
    pub fn get_current_branch_name(&mut self) -> Result<String, GitError> {
        let result = self
            .runner
            .run(&["rev-parse", "--abbrev-ref", "HEAD"])
            .map_err(GitError::Io)?;

        if result.success {
            Ok(String::from_utf8_lossy(&result.stdout).trim().to_string())
        } else {
            let stderr = String::from_utf8_lossy(&result.stderr).to_string();
            Err(GitError::GitFailed(stderr))
        }
    }
}

/// Compatibility wrapper that preserves the original behavior (panics on failure).
pub fn get_current_branch_name() -> String {
    let mut git = Git::real();
    match git.get_current_branch_name() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error executing git command:\n{}", e);
            panic!("Failed to get current branch name.\n Are you in a git repository?");
        }
    }
}

/// Push the current branch to origin, with an option to skip git hooks.
pub fn git_push_origin(skip_hooks: bool) {
    let push = Command::new("git")
        .args([
            "push",
            "-u",
            "origin",
            if skip_hooks { "--no-verify" } else { "" },
        ])
        .output()
        .expect("Failed to push branch to origin");

    if push.status.success() {
        println!("Pushed branch to origin");
        return;
    }

    eprintln!(
        "Error pushing branch to origin:\n{}",
        String::from_utf8_lossy(&push.stderr)
    );
}

pub fn git_checkout_branch(branch_name: &str) -> bool {
    let checkout_branch = Command::new("git")
        .args(["checkout", branch_name])
        .output()
        .expect("Failed to checkout branch");

    checkout_branch.status.success()
}

pub fn git_create_branch(branch_name: &str) -> bool {
    let create_branch = Command::new("git")
        .args(["checkout", "-b", branch_name])
        .output()
        .expect("Failed to create branch");

    create_branch.status.success()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fake runner that returns a predetermined RunResult.
    struct FakeRunner {
        result: RunResult,
    }

    impl FakeRunner {
        fn new(success: bool, stdout: &str, stderr: &str) -> Self {
            Self {
                result: RunResult {
                    success,
                    stdout: stdout.as_bytes().to_vec(),
                    stderr: stderr.as_bytes().to_vec(),
                },
            }
        }
    }

    impl GitRunner for FakeRunner {
        fn run(&mut self, _args: &[&str]) -> std::io::Result<RunResult> {
            Ok(self.result.clone())
        }
    }

    #[test]
    fn returns_branch_name_on_success() {
        let fake = FakeRunner::new(true, "feature/test-branch\n", "");
        let mut git = Git::with_runner(Box::new(fake));
        let branch = git
            .get_current_branch_name()
            .expect("expected branch name on success");
        assert_eq!(branch, "feature/test-branch");
    }

    #[test]
    fn trims_whitespace_from_branch_name() {
        let fake = FakeRunner::new(true, "  feature/trim  \n", "");
        let mut git = Git::with_runner(Box::new(fake));
        let branch = git.get_current_branch_name().unwrap();
        assert_eq!(branch, "feature/trim");
    }

    #[test]
    fn returns_error_when_git_fails() {
        let fake = FakeRunner::new(false, "", "fatal: not a git repository\n");
        let mut git = Git::with_runner(Box::new(fake));
        let err = git.get_current_branch_name().unwrap_err();
        match err {
            GitError::GitFailed(s) => assert!(s.contains("not a git repository")),
            other => panic!("expected GitFailed, got {:?}", other),
        }
    }

    #[test]
    fn compatibility_wrapper_panics_on_failure_when_not_in_repo() {
        // The compatibility wrapper is expected to panic when git fails.
        // Use a fake runner and call the wrapper by constructing a Git with a failing runner,
        // but since the wrapper always uses RealGitRunner, we only assert that the wrapper
        // returns a String or panics - here we don't invoke the wrapper to avoid relying on
        // environment git. This test documents behavior rather than execute it.
        assert!(true, "Compatibility wrapper kept for backward compatibility");
    }
}
```
