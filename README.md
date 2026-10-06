# cli-timer

Um cronômetro de terminal escrito em Rust. É possível criar vários cronômetros com nomes diferentes, iniciá-los, pausá-los, consultá-los, zerá-los e removê-los.

> Você não precisa conhecer Rust para usar o projeto. Basta instalar o Rust uma vez e seguir os passos abaixo.

## Requisitos

- Windows 10 ou mais recente, ou uma distribuição Linux;
- Git, para baixar o projeto;
- Rust e Cargo, instalados pelo `rustup`.

O Cargo é o gerenciador de projetos e de compilação do Rust. A instalação do Rust já inclui o Cargo.

## 1. Instale o Rust

### Windows

1. Instale o **Visual Studio Build Tools** com o pacote **Desenvolvimento para Desktop com C++**. Ele fornece o linker necessário para compilar o projeto.
2. Baixe e execute o instalador oficial em [rustup.rs](https://www.rust-lang.org/tools/install).
3. Quando o instalador perguntar, escolha a opção padrão (`1`).
4. Feche e abra o PowerShell novamente.

Confirme a instalação:

```powershell
rustc --version
cargo --version
```

### Linux

Abra um terminal e instale o compilador C e as ferramentas básicas da sua distribuição. Em Ubuntu, Debian e Linux Mint:

```bash
sudo apt update
sudo apt install build-essential curl git
```

Depois, instale o Rust com o instalador oficial:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Escolha a instalação padrão (`1`). Ao terminar, carregue o ambiente no terminal atual:

```bash
source "$HOME/.cargo/env"
```

Confirme a instalação:

```bash
rustc --version
cargo --version
```

## 2. Baixe o projeto

No terminal, escolha uma pasta e execute:

```bash
git clone <URL_DO_REPOSITORIO>
cd cli-timer
```

No PowerShell, os mesmos comandos funcionam:

```powershell
git clone <URL_DO_REPOSITORIO>
cd cli-timer
```

Se você já recebeu a pasta do projeto, basta abrir o terminal dentro dela. Para conferir se está na pasta correta, procure por `Cargo.toml`:

```bash
# Linux
ls Cargo.toml

# Windows PowerShell
Get-Item Cargo.toml
```

## 3. Compile e rode

Na primeira execução, o Cargo baixa as dependências e pode levar alguns minutos. Dentro da pasta do projeto:

```bash
cargo run -- list
```

Se tudo estiver certo, a saída inicial será:

```text
No stopwatches found.
```

O `--` separa as opções do Cargo dos comandos do cronômetro. Ele deve ser mantido nos comandos `cargo run`.

## Uso básico

Sem informar um nome, o programa usa o cronômetro chamado `default`:

```bash
cargo run -- start
cargo run -- status
cargo run -- pause
cargo run -- stop
```

Para trabalhar com cronômetros nomeados, informe o nome depois do comando:

```bash
cargo run -- start estudos
cargo run -- status estudos
cargo run -- pause estudos
cargo run -- reset estudos
cargo run -- delete estudos
```

Comandos disponíveis:

| Comando | O que faz |
| --- | --- |
| `start [nome]` | inicia um cronômetro; cria-o se necessário |
| `pause [nome]` | pausa o cronômetro sem perder o tempo acumulado |
| `stop [nome]` | para e zera o cronômetro |
| `status [nome]` | mostra um cronômetro específico ou todos |
| `reset [nome]` | zera o tempo, mas mantém o cronômetro salvo |
| `delete [nome]` | remove o cronômetro |
| `list` | lista todos os cronômetros |

Para ver a ajuda completa:

```bash
cargo run -- --help
```

### Nomes com espaços

Coloque o nome entre aspas:

```bash
# Linux
cargo run -- start "leitura diária"

# Windows PowerShell
cargo run -- start "leitura diária"
```

## Compilar sem o Cargo executar o programa

Para gerar uma versão de desenvolvimento:

```bash
cargo build
```

Para gerar uma versão otimizada para uso normal:

```bash
cargo build --release
```

Depois, execute o binário diretamente:

```bash
# Linux
./target/release/cli-timer list
./target/release/cli-timer start estudos

# Windows PowerShell
.\target\release\cli-timer.exe list
.\target\release\cli-timer.exe start estudos
```

Embora o executável se chame `cli-timer`, a ajuda e as mensagens do programa usam o nome `stopwatch`.

## Onde os dados ficam salvos?

O programa cria o arquivo `.stopwatch.json` na **pasta em que o comando foi executado**. Esse arquivo guarda os nomes, os tempos acumulados e o estado dos cronômetros.

Por isso:

- execute o programa sempre a partir da pasta do projeto se quiser reutilizar os mesmos dados;
- não apague `.stopwatch.json` se quiser manter os cronômetros;
- para começar do zero, feche o programa e apague esse arquivo manualmente.

No PowerShell:

```powershell
Remove-Item .stopwatch.json
```

No Linux:

```bash
rm .stopwatch.json
```

## Problemas comuns

### `cargo` ou `rustc` não foi encontrado

Feche e abra o terminal depois de instalar o Rust. No Linux, tente novamente:

```bash
source "$HOME/.cargo/env"
```

Se ainda não funcionar, reinstale o Rust pelo [rustup](https://rustup.rs/).

### Erro de linker no Windows

Instale o **Visual Studio Build Tools** com o workload **Desenvolvimento para Desktop com C++** e abra um novo PowerShell.

### A compilação falha no Linux

Confira se as ferramentas básicas estão instaladas:

```bash
sudo apt update
sudo apt install build-essential
```

Em outras distribuições, instale o equivalente a GCC/Clang, `make` e as bibliotecas padrão de desenvolvimento.

### O cronômetro parece ter sumido

Verifique se o comando está sendo executado em outra pasta. O arquivo `.stopwatch.json` é relativo à pasta atual, não à pasta do código-fonte.

## Desenvolvimento

Para verificar se o código compila:

```bash
cargo check
```

Para executar os testes, quando existirem:

```bash
cargo test
```
