> ---
> # Índice

- [COOKBOOK: Aprendendo com testes](#cookbook-aprendendo-com-testes)
  - [Criando um comando para ler o nome da branch](#criando-um-comando-para-ler-o-nome-da-branch)
    - [Contexto](#contexto)
    - [Sobre a implementação](#sobre-a-implementação)
    - [TDD: Primeira iteração - Mock FakeRunner e RunResult](#tdd-primeira-iteração---mock-fakerunner-e-runresult)
      - [Explicando o código](#explicando-o-código)
      - [Implementando pub struct RunResult](#implementando-pub-struct-runresult)
      - [Finalizando a iteração](#finalizando-a-iteração)
      - [Código resultante deta iteração](#código-resultante-deta-iteração)
    - [TDD: Segunda iteração - GitRunner](#tdd-segunda-iteração---gitrunner)
      - [Vamos entender as mudanças:](#vamos-entender-as-mudanças)
      - [Implementando pub trait GitRunner](#implementando-pub-trait-gitrunner)
      - [Finalizando a iteração](#finalizando-a-iteração-1)
      - [Código resultante deta iteração](#código-resultante-deta-iteração-1)
    - [TDD: Terceira iteração - Git, the real implementation](#tdd-terceira-iteração---git-the-real-implementation)
      - [Explicando o código:](#explicando-o-código-1)
      - [Implementando RealGitRunner e impl Git](#implementando-realgitrunner-e-impl-git)
      - [Finalizando a iteração](#finalizando-a-iteração-2)
      - [Código resultante deta iteração](#código-resultante-deta-iteração-2)
    - [TDD: Quarta iteração - GitError, melhorando as mensagens de erros](#tdd-quarta-iteração---giterror-melhorando-as-mensagens-de-erros)
      - [Código resultante deta iteração](#código-resultante-deta-iteração-3)
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

## Criando um comando para ler o nome da branch

> #test #command #mock

### Contexto

Estou criando uma função que irá executar um commando no terminal para saber qual o nome da branch do git que estou no momento.

O comando git para isso é: `git rev-parse --abbrev-ref HEAD`.

### Sobre a implementação

Para escrevermos o teste desta execução, precisaremos encapsular a execução do comando de forma que possa receber o executável como parâmetro.  
Isso será usado para podemos efetuar um mock do executável.

> Mock:
> Mocks são simulações de algo real, utilizados principalmente para em testes de softwares onde substituem aluma dependência real. Podendo assim, controlar os resultados dessa dependência.

Então, vamos implementar nosso teste primeiro.

### TDD: Primeira iteração - Mock FakeRunner e RunResult

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

#### Explicando o código

Assinatura do teste:

```rs
#[cfg(test)]
mod tests {
    // código
}
```

A definição do teste se da através do atributo de configuração `cfg()`.  
Essa definição de atributo de configuração (`cfg`) indicará ao compilador que
**mod tests** na linha seguinte é relacionada aos testes e precisa ser compilado somente quando
executamos os testes através de `cargo test` executado no terminal.

O termo `mod` indica um que estamos definindo um novo módulo. 
Se compararmos com HTML, seria como abrir uma nova tag dentro de outra:

```html
<arquivo>
  <mod name="tests"></mod>
</arquivo>
```

Sendo o arquivo também considerado um módulo, o `mod tests` indica que estamos definindo um submódulo, um módulo dentro de outro.  
A ideia do HTML é interessante de se trazer porque temos també um conceito similar: `parent`.

O que nos leva ao seguinte código abaixo

```rs
#[cfg(test)]
mod tests {
    use super::*;
    // código
}
```

A palavra chave `use` significa que estamos importando (usando) algo de outro módulo.  
Em outras linguages de programação temos a palavra `import`, por exemplo.

A palavra chave `super` acessa o módulo parente, acima, mais próximo, mais externo.  
A analogia com html faz um sentido melhor aqui. Quando executa `use super::*` estamos importando
tudo do nosso `parent`.  
Que no nosso exemplo, seria o arquivo onde implementaremos nosso código.  
Em visão orientada a objetos, seria como usar extender de outra classe. 
Veja os exemplo abaixo.

<details>
<summary>Exemplos de códigos em outras linguagens</summary>

```java
// JAVA
class ParentClass {
  public ParentClass() {}
  public void sum() {}
  public void minus() {}
}

class ChildClass extends ParentClass {}

public static void main(String[] args) {
  ChildClass obj = new ChildClass();
  obj.sum();
}
```

```python
class ParentClass:
    def __init__(self):
        pass

    def sum(self):
        pass

    def minus(self):
        pass

class ChildClass(ParentClass):
    pass

def main():
    obj = ChildClass()
    obj.sum()


if __name__ == "__main__":
    main()
```

</details>

Se não desejar manter o teste dentro do mesmo arquivo, é possível separar o teste usando a seguinte estrutura.

Sendo o arquivo a ser testado nomeado como `git.rs`:

1. Cria uma pasta com o mesmo nome, no mesmo nível do arquivo. Exemplo: se `src/git.rs`, então temos `src/git/`.
2. Cria um arquivo tests.rs dentro da pasta criada. Exemplo: `src/git/tests.rs
3. Ao fim do arquivo `git.rs` adicione a linhas abaixo, identificand que o submodulo chamado tests existe e está configurado para testes.
```rs
#[cfg(test)]
mod tests;
```

Assim em `src/git/tests.rs` é possível acessar todos os atributos, inclusive os privados, com `use super::*`.

```rs
#[cfg(test)]
mod tests {
    // Restante do código

    struct FakeRunner {
        result: RunResult,
    }
}
```

Neste trecho do código temos a palavra chave `struct`. Ela é utilizada para definir uma estrutura.  
Structs podem ser utilizadas para definir diferentes coisas, orientando-se pelo conceito de chave e valor. Em outras linguages, como `typescript`, `struct` pode ser comparado com `types`. Ou em `java` ser comparado a um `record`.

> Nota, as comparações feita são para facilitar analogias, não para definir o conceito.

Assim, o código acima define uma estrutura (`struct`) chamada `FakeRunner`. Onde nela temos um valor chamado `result`, cujo o tipo é `RunResult`.  
Nada disso já foi definido, esse é o intuito do TDD, dizer o que queremos nos testes,
para só depois implementarmos o que desejamos.

Neste caso, RunResult será outra estrutura que iremos definir posteriormente.

```rs
#[cfg(test)]
mod tests {
    // Restante do código

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

Aqui temos outra palavra chave `impl`. Esta palavra chave tem um significado semântico para **implementação**.  
Logo podemos ler esta linha de código como: `impl`ementação para `FakeRunner`.  
Mas, FakeRunner só tem um valor definido, result, e aqui vemos uma função, como assim?

`impl` tem mais um papel de adicionar funcionalidade do que modificar, então imagine que você está adicionando uma funcionalidade a struct `FakeRunner`.  
Então, neste caso estamos adicionado a função `new` a struct FakeRunner.

Para definir uma função utilizamos a palavra chave `fn` seguira pelo nome e os argumentos desejados, e por fim o tipo do retorno.  
No nosso exemplo temos `fn new(success: bool, stdout: &str, stderr: &str) -> Self {`.  
`fn` define que estamos definindo uma função. 
`new` é p nome da funcão que estamos implementando
`(success: bool, stdout: &str, stderr: &str)` são os argumentos necessários para a função.
Aqui temos 3 argumentos, `success` que é do tipo bool (sim ou não, true or false, verdadeiro ou falso); `stdout: &str` e `stderr: &str` são é um atributo de texto (string) que não tomam posse do valor enviado (conceito de [borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html));

Também temos o retorno `-> Self`, isso significa que o tipo de retorno é a própria struct.  
Qual a vantagem disso? Podemos fazer chamadas concatenadas como por exemplo se tivessimos uma calculadora, poderiamos executar: `Calculadora::new(10).mais(5).menos(2).restultado()`. Se resultado imprimir a resposta, teríamos o resultado **13**, pois iniciamos a calculadora com o valor de **10**, depois somamos **5**, e depois subtraímos **2**, resultando em **13**.

O mesmo `Self` é utilizado dentro do método `new()`. Isso significa que o retorno do método altera algo interno da estrutura (struct) FakeRunner.  
Nesse exemplo que retornamos a própria struct com o valor de result alterado.

```rs
Self {
    result: RunResult {
        success,
        stdout: stdout.as_bytes().to_vec(),
        stderr: stderr.as_bytes().to_vec(),
    },
}
```

O valor de result será uma `struct RunResult`, que possue em seus campos utilizamos os argumentos enviados na chamada da funcão.  
`stdout` e `stderr` recebem os valores no formato de um lista de bytes, por terem sidos definidos como `str`, logo `.as_bytes().to_vec()` converte primeiro o valor para bytes e depois transforma isso em uma lista de bytes.  
Como ainda não implementamos `RunResult` fica estranho ver toda essa conversão acontecendo aqui. Mas, lembre-se implementamos as solução pensando nos testes, e não os teste pensando na solução. Assim, desejamos que a struct RunResult seja definida com stdout e stderr como uma lista de bytes.

A seguir, vamos finalmente implementar nossa estrutura RunResult.
Ela faz parte da nossa implementação final, logo será definida fora do modulo de testes.

#### Implementando pub struct RunResult

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

- `#[derive(Clone, Debug)]`: 
  - se você entende de Orientação a Objetos, podemos fazer um comparativo dizendo que estamos herdando, nesse caso derivando, os métodos Clone e Debug nativos da linguagem.  
  - Isso significa que a struct criada terá os métodos `clone()` e `debug()`, utilizando da macro `derive`.
- `pub struct RunResult {`:
  - A definição da struct como pública, permitindo uso fora do módulo (ou simplesmente do arquivo).  
  - Em Rust, por padrão, tudo é privado, logo precisamos por a notação `pub` em tudo que queremos ter acesso fora do módulo. Acredito que seja uma decisão de segurança :)

#### Finalizando a iteração

Execute os testes e vamos ver se eles passam:

```sh
cargo test
```

Com isso implementado, vamos então fazer nosso primeiro commit: 

```sh
git add .; 
git commit -m "feat: wip - implementando estrutura do resultado da consulta do git"
```

#### Código resultante deta iteração

```rs
#[derive(Clone, Debug)]
pub struct RunResult {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

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

### TDD: Segunda iteração - GitRunner

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

#### Vamos entender as mudanças:

- `impl GitRunner for FakeRunner {`: **impl**ementação de **GitRunner** **para** a estrutura **FakeRunner**. Essa linha indica que estamos implementando uma definição (trait) para estrutura `FakeRunner`. Em orientação a objeto seria como um `extends` ou `implements` e `GitRunner` seria considerada como uma `interface`. A forma como `impl` funciona é interessante, é como se fossemos adicionando novas funcionalidades a estrutura FakeRunner cada vez que utilizamos. A primeira vez, adicionamos new, da segunda vez, utilizamos uma definição (trait) para dizer que função deveriamos implementar.

```rs
fn run(&mut self, _args: &[&str]) -> std::io::Result<RunResult> {
    Ok(self.result.clone())
}
```

- O código acima representa a função definida no trait `GitRunner`. A assinatura do método, o tipo de retorno, tudo é definido dentro de `GitRunner`. Mas, através de `impl GitRunner for ...` é quando a função realmente é implementada, deixando de ser somente uma assinatura.  
- Nesse caso, nós simplesmente retornamos uma cópia do objeto result, que é definido na função `new`.


Agora vamos implementar `GitRunner` trait que é utilizado como definição para `FakeRunner`

#### Implementando pub trait GitRunner

```rs
pub trait GitRunner {
    fn run(&mut self, args: &[&str]) -> std::io::Result<RunResult>;
}
```

Vamos entender o código acima:

- `pub trait GitRunner {`:
  - Assim como para struct, `pub` é utilizado para definir esta trait como publica, com acesso externo.  
- `fn run(&mut self, args: &[&str]) -> std::io::Result<RunResult>;`:
  - é a definição da função `run`.
  - `&mut self` 
    - significa que estamos passando uma referência que pode ser alterada de "si mesmo",  
  - `args: &[&str]` 
    - significa que podemos receber um array sem limites de valores de texto (string)  
  - `std::io::Result<RunResult>;` 
    - é outra forma de escrever `Result<RunResult, std::io::Error>`. 
    - Isto é usado como retorno de funções IO (input/output, entrada/saída), como é no nosso caso, onde iremos executar um commando git'. 
    - O retorno é um `Ok(RunResult)` caso de sucesso, retornando um resultado na estrutura de `RunResult`; Ou um valor de erro `Err(std::io::Error)` do tipo I/O. 
    - Esse formato é comumento usado para implementar `?` para propagação de error, bem como funções que executan I/O ou chamam outras API com retorno I/O.

Assim como em Orientação a Objeto, na qual temos uma `interface` que é uma abstração de uma implementação real, e então implementamos uma `classe` que implementa esta `interface` de modo a termos uma classe concreta, em rust faremos algo semelhante.

#### Finalizando a iteração

Vamos então executar nosso teste: 

```sh
cargo test
```

Tudo estando OKAY, podemos fazer nosso segundo commit:

```sh
git add .; 
git commit -m "feat: wip implementando trait GitRunner"
```

#### Código resultante deta iteração

```rs
#[derive(Clone, Debug)]
pub struct RunResult {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub trait GitRunner {
    fn run(&mut self, args: &[&str]) -> std::io::Result<RunResult>;
}

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

    impl GitRunner for FakeRunner {
        fn run(&mut self, _args: &[&str]) -> std::io::Result<RunResult> {
            Ok(self.result.clone())
        }
    }
}
```

### TDD: Terceira iteração - Git, the real implementation

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

#### Explicando o código:

- `#[test]`: 
  - Esse atributo indica que a função seguinte é um teste  
- `fn returns_branch_name_on_success() {`: 
  - Assinatura da função, o nome da função será o nome exibido no terminal.
- `let fake = FakeRunner::new(true, "feature/test-branch\n", "");`: 
  - nesta linha inicializamos o nosso mock runner. 
  - Nele passamos o valor de sucesso e o nome da branh na qual querermos retornar, o terceiro parâmetro é de erro, não necessário neste teste.  
- `let mut git = Git::with_runner(Box::new(fake));`: 
  - Nesta linha implementamos o nossa implementação concreta do trait GitRunner. Iremos implementar a seguir, logo, o teste falhará por isso.

```rs
let branch = git
    .get_current_branch_name()
    .expect("expected branch name on success");
```

- O código acima executa o método de git.
- `assert_eq!(branch, "feature/test-branch");`: 
  - aqui temos a nossa validação no teste. Esta linha que indica se o teste foi sucesso.

Agora que entendemos o teste implementado, vamos escrever nosso código para passar

#### Implementando RealGitRunner e impl Git

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

Explicando o código

- `impl GitRunner for RealGitRunner {`: 
  - **implementar** o trait **GitRunner** **para** a estrutura **RealGitRunner**. 
    - Parece lógico né? Estamos implementando um `trait` em uma `struct`.
- `fn run(&mut self, args: &[&str]) -> std::io::Result<RunResult> {`:
  - veja que contém a mesma assinatura que o método do trait, isso significa que esta é a implementação real da função.
- `let output = Command::new("git").args(args).output()?;`:
  - Esta linha executa o comando do **git** passando a lista de argumentos.

```rs
Ok(RunResult {
    success: output.status.success(),
    stdout: output.stdout,
    stderr: output.stderr,
})
```

- As linhas acimas são o retorno de sucesso da chamada da função. 
- Para nosso exemplo, não precisaremos de um retorno de falha `Err()`, pois `output()?` já faz esse papel.

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

- O código acima define a estrutura do Git.  
- `runner: Box<dyn GitRunner>,`: 
  - Alocação de ponteiro-heap para algum tipo concreto de uma implementação do GitRunner, mas que em tempo de compilação está vazio, pois sua alocação se dá em tempo de execução. **Box<>** é um trait que aloca um espaço na memória heap e armazena um ponteiro próprio para o conteúdo. **dyn GitRunner**, é um trait para objeto que habilita uso dinâmico através de uma [vtable](https://users.rust-lang.org/t/v-tables-differences-between-rust-and-c/92445/2) em tempo de execução.

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

- `impl Git {`: 
  - Inicialização da implementação da struct Git  
- `pub fn real() -> Self {`: 
  - Esse método retorna uma implementação "verdadeira" para o uso real. O nome da funcão ser `real()` é uma convenção. Isso indica que este método não retorna um objeto mockado ou alguma versão difereciada.

```rs
Self {
    runner: Box::new(RealGitRunner),
}
```

Essa parte do método `real()` executa um acesso a variável definida na `struct`. Este termo `Self` indica que estou dentro de um contexto interno.  
Sendo assim, o método altera o o valor de `runner` com um novo `Box` usando `RealGitRunner` como execultável

- `pub fn with_runner(runner: Box<dyn GitRunner>) -> Self {`: 
  - este é o maravilhoso método usado no teste. Ele aceita um parâmetro do tipo `Box<dyn GitRunner>` e define ele no runner. Em outras linguagens de programação isso poderia ser chamado de um método `set`, mas por legibilidade `with_algumacoisa()` tem uma melhor legibilidade, sobretudo em chamadas de métodos concatenados.  
- `Self { runner }`: 
  - assim, o método somente tem uma definição direta do runner. Como o nome do parâmetro da função é igual ao valor na struct, não se faz necessário escrever `runner: runner`.

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

- `pub fn get_current_branch_name(&mut self) -> Result<String, String> {`: 
  - Aqui temos a assinatura do método onde o parâmetro da função é um `&mut self`. 
  - **&mut self** aqui existe por conta da implementação dinâmica do runner. Como a definição do runner não é feita em tempo de compilação, a consequência disso é que precisamos dizer ao nosso método que `self` pode ser alterado a qualquer momento. 
  - **Result<String, GitError>** significa que iremos retornar um `Ok()` (para sucesso) ou um `Err()` (para falha), onde o retorno de Ok é uma string e o de Err é um tipo String (Isso irá mudar na próxima iteração para um erro mais customizado).  

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

- `&result.stdout`: 
  - Contém o resultado do comando em um formato de `bytecode`, um `Vec<u8>`. 
  - O `&` indica que estamos passando uma referência ao valor, chamamos isso de [`borrowing`](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html). 
    - **Borrowing** é um conceito muito necessário em Rust, vale a pena dedicar um tempo lendo sobre.
- `String::from_utf8_lossy(&result.stdout)`: 
  - Converte os bytes em um texto UTF-8. Usamos este método para converter bytes para texto UTF-8 com tolerância a falha, caso dé erro, caracteres como `�` são inseridos como `texto`
- `.trim()`: 
  - remove espaços em branco no início e fim.
- `.to_string()`: 
  - Converte o resultado em um String alocado (owned). Isso garante que quem chama o método irá deter posse do resultado ([ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)). 
    - **Ownership** é outro conceito de extrema importância em rust. 

> Em um resumo nada convencional de explicar, ownership e borrowing são os meios do rust de fazer com que a pessoa desenvolvedora seja responsável pelo garbage collector :D


#### Finalizando a iteração

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

#### Código resultante deta iteração

```rs
#[derive(Clone, Debug)]
pub struct RunResult {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub trait GitRunner {
    fn run(&mut self, args: &[&str]) -> std::io::Result<RunResult>;
}

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

    impl GitRunner for FakeRunner {
        fn run(&mut self, _args: &[&str]) -> std::io::Result<RunResult> {
            Ok(self.result.clone())
        }
    }

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
}
```


### TDD: Quarta iteração - GitError, melhorando as mensagens de erros

Até a terceira iteração, já tinhamos o código funcionando e testado.  
Mas, as mensagens de erros devem ser melhor trabalhadas.  
A pior coisa para um desenvolvedor é ver um problema e não ter a menor ideia do
motivo por traz daquilo.

Então, se iremos implementar uma rotina de teste mais bem elaborada, vamos então
escrever os testes para isso. A explicação está condiga no próprio código, através de comentários `//`

```rs
#[cfg(tests)]
mod test {
    /**
     * Código anterior já implementado, somente a primeira linha :D
    use super::*;
    struct FakeRunner {
    impl FakeRunner {
    impl GitRunner for FakeRunner {
    #[test]
    fn returns_branch_name_on_success() {
     */

    /**
     * Implementação do teste para quando falhar
     */
    // Definição da função de teste com o nome que será exibido ao rodar cargo tests
    #[test]
    fn returns_error_when_git_fails() {
        // Aqui temos a implementação do mock
        // Observe o detalhe que o segundo parâmetro está vazio, isso indica que no 
        //    caso de retorno OK será um resultado vazio.
        // Logo o retorno desejado está no terceiro parâmetro. Onde na implementação do mock
        //    corresponde ao retorno do erro.
        let fake = FakeRunner::new(false, "", "fatal: not a git repository\n");
        // Nesta linha é inicializada a implementação do Git passando o mock runner acima.
        let mut git = Git::with_runner(Box::new(fake));
        // Aqui temos a utilização da funcionalidade do match.
        // Em outras linguages o match se acemelha ao swithc-case. Na tradução literal match = corresponder.
        // Logo, na linha abaixo informamos que o retorno da função conrresponde a algum dos resultados listados
        // Esta é uma excelente forma de efetuar validações de diferentes casos de retorno.
        match git.get_current_branch_name() {
            // Na linha abaixo estamos correspondendo o retorno de get_current_branch_name
            //    ao tipo Err(GitError::GitFailed(msg)). Aqui vemos o teste da nossa nova implementação
            Err(GitError::GitFailed(msg)) => {
                assert!(msg.contains("not a git repository"));
            }
            // Nas duas linhas abaixo retornamos o teste usando panic!
            // panic! é uma macro que dispara um evento de falha do projeto.
            // Este erro interrompe a execução do programa, e exibe a mensagem definida.
            // Estes dois casos abaixos indicam que o nosso teste deve obrigatoriamente retornar
            //    um tipo Err(GitError::GitFailed(msg)), do contrário algo de errado não está certo.
            Err(e) => panic!("expected GitFailed, got {:?}", e),
            Ok(v) => panic!("expected error, got success {:?}", v),
            _ => panic!("expected error, got success {:?}", v),
        }
    }
}
```



<details>

  <summary>Sobre o match e sua semalhança com `switch-case` de outras linguagens</summary>

  O [`match`](https://doc.rust-lang.org/book/ch06-02-match.html) pode ser utilizado em diversos casos, 
  não só no tratamento de retorno. Ele é uma estrutura de controle, assim como if-else.  
  Em outras linguagens, seria o mesmo que um switch case, veja abaixo:

  ```php
  // Javascript ou Typescript ou Java ou Php
  switch (expression) {
    case value1:
      // Code to execute if expression === value1
      break;
    case value2:
      // Code to execute if expression === value2
      break;
    // ... more cases
    default:
      // Code to execute if no case matches
  }
  ```

  Já python, a partir da versão 3.10 temos algo semelhante ao match de rust:

  ```python
  match choice:
      case 'A':
          print("You selected option A.")
      case 'B':
          print("You selected option B.")
      case 'C':
          print("You selected option C.")
      case _:  # Wildcard for default case
          print("Unknown selection.")
  ```

  ```rs
  match number {
      1 => println!("The number is one."),
      2 | 3 => println!("The number is two or three."), // Matching multiple values
      4..=6 => println!("The number is between four and six (inclusive)."), // Matching a range
      _ => println!("The number is something else."), // Catch-all pattern
  }
  ```

</details>

#### Código resultante deta iteração

```rs
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

#[derive(Clone, Debug)]
pub struct RunResult {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub trait GitRunner {
    fn run(&mut self, args: &[&str]) -> std::io::Result<RunResult>;
}

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

    #[test]
    fn returns_error_when_git_fails() {
        let fake = FakeRunner::new(false, "", "fatal: not a git repository\n");
        let mut git = Git::with_runner(Box::new(fake));
        match git.get_current_branch_name() {
            Err(GitError::GitFailed(msg)) => {
                assert!(msg.contains("not a git repository"));
            }
            Err(e) => panic!("expected GitFailed, got {:?}", e),
            Ok(v) => panic!("expected error, got success {:?}", v),
            _ => panic!("expected error, got success {:?}", v),
        }
    }
}

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

    impl GitRunner for FakeRunner {
        fn run(&mut self, _args: &[&str]) -> std::io::Result<RunResult> {
            Ok(self.result.clone())
        }
    }

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
}
```

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
        match git.get_current_branch_name() {
            Err(GitError::GitFailed(msg)) => {
                assert!(msg.contains("not a git repository"));
            }
            Err(e) => panic!("expected GitFailed, got {:?}", e),
            Ok(v) => panic!("expected error, got success {:?}", v),
        }
    }
}
```
