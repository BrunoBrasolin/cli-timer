# cli-timer

Um cronômetro web escrito em Rust para contar horas de trabalho em projetos diferentes. O programa sobe um pequeno servidor local que serve uma página com uma tabela de todos os cronômetros, onde é possível criar, iniciar, pausar, parar, zerar e remover cada um deles.

O projeto é 100% Rust: o servidor HTTP usa apenas a biblioteca padrão, e a página é gerada pelo próprio programa, sem JavaScript, Node ou arquivos externos.

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
cargo run
```

Se tudo estiver certo, a saída será:

```text
Stopwatch running at http://127.0.0.1:3000
Press Ctrl+C to stop.
```

Abra [http://127.0.0.1:3000](http://127.0.0.1:3000) no navegador. Para encerrar o servidor, pressione `Ctrl+C` no terminal.

### Usar outra porta

Defina a variável de ambiente `PORT`:

```bash
# Linux
PORT=8080 cargo run

# Windows PowerShell
$env:PORT=8080; cargo run
```

O servidor escuta apenas em `127.0.0.1`, ou seja, só pode ser acessado pelo próprio computador.

## Uso

A página mostra uma tabela com todos os cronômetros salvos em `.stopwatch.json`, com nome, estado, tempo decorrido e ações. Os cronômetros em andamento são atualizados na tela a cada segundo. A última linha mostra o total de todos os cronômetros.

Para criar um cronômetro, clique em **+ New timer**, digite o nome e clique em **Create**. Nomes com espaços e acentos funcionam normalmente.

Ações de cada linha:

| Botão | O que faz |
| --- | --- |
| **Start** | inicia o cronômetro, continuando do tempo acumulado |
| **Pause** | pausa o cronômetro sem perder o tempo acumulado |
| **Stop** | para o cronômetro e zera o tempo |
| **Reset** | zera o tempo; se o cronômetro estiver rodando, ele continua rodando a partir de zero |
| **Delete** → **Confirm** | remove o cronômetro (pede confirmação) |

Os cronômetros continuam contando mesmo com a página ou o servidor fechados, pois o arquivo guarda o horário em que cada um foi iniciado.

## Compilar sem o Cargo executar o programa

Para gerar uma versão otimizada para uso normal:

```bash
cargo build --release
```

Depois, execute o binário diretamente:

```bash
# Linux
./target/release/cli-timer

# Windows PowerShell
.\target\release\cli-timer.exe
```

## Onde os dados ficam salvos?

O programa cria o arquivo `.stopwatch.json` na **pasta em que o servidor foi iniciado**. Esse arquivo guarda os nomes, os tempos acumulados e o estado dos cronômetros.

Por isso:

- execute o programa sempre a partir da pasta do projeto se quiser reutilizar os mesmos dados;
- não apague `.stopwatch.json` se quiser manter os cronômetros;
- o arquivo é lido a cada vez que a página é carregada, então alterações manuais aparecem ao recarregar a página;
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

### A porta já está em uso

Se aparecer `Failed to start server on 127.0.0.1:3000`, outro programa já está usando a porta. Use outra, por exemplo `PORT=8080 cargo run`.

### O cronômetro parece ter sumido

Verifique se o servidor foi iniciado em outra pasta. O arquivo `.stopwatch.json` é relativo à pasta atual, não à pasta do código-fonte.

## Desenvolvimento

Estrutura do código:

| Arquivo | Responsabilidade |
| --- | --- |
| `src/main.rs` | lê a porta e inicia o servidor |
| `src/server.rs` | servidor HTTP e rotas (`GET /`, `POST /timers`, `POST /timers/action`) |
| `src/page.rs` | gera o HTML e o CSS da página |
| `src/stopwatch.rs` | regras do cronômetro (iniciar, pausar, parar, zerar) |
| `src/storage.rs` | leitura e gravação de `.stopwatch.json` |

Para verificar se o código compila:

```bash
cargo check
```

Para executar os testes, quando existirem:

```bash
cargo test
```
