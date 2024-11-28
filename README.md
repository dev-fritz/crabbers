# Crabbers

- Um conjunto de serviços utilitários escritos em Rust, prontos para simplificar tarefas comuns no dia a dia.

## Serviços disponíveis

### 1. Crab - Network

- Verificação de velocidade da Internet: realize testes de upload e download para monitorar sua conexão.
- Informações de rede: obtenha dados como IP interno, IP externo, e detalhes da rede local.

### 2. Crab - Service

- Gerenciamento de serviços: escolha e execute serviços específicos de forma prática via linha de comando, inspirado pela ideia de uma CLI.

### 3. Crab - PDF

- Conversão para PDF: converta arquivos de diversos formatos (HTML, Word, imagens, etc.) para PDF.
- Manipulação de PDF:
  - Divida um PDF grande em partes menores.
  - Comprima PDFs para reduzir o tamanho do arquivo.

### 4. Crab - Crypt

- Criptografia e Segurança:
  - Criptografe e descriptografe arquivos de maneira simples.
  - Gere hashes (como MD5, SHA256) para verificação de integridade.
  - Proteja PDFs com senha ou remova proteções de senha, se permitido.

## Como executar

- Clone o repositório:
    
  ```bash
  git clone https://github.com/seu-usuario/crabbers.git
  cd crabbers
  ```

### Instale as dependências e compile o projeto:

  ```bash
  cargo build --release
  ```

### Use os comandos para cada serviço:
```bash
    Crabbers: cargo run
    Crab - Network: cargo run -- network
    Crab - Service: cargo run -- service
    Crab - PDF: cargo run -- pdf
    Crab - Crypt: cargo run -- crypt
```