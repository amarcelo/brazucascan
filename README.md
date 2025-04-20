# Documentação do Brazucascan

## 1. Visão Geral
O **Brazucascan** é uma aplicação de linha de comando desenvolvida em Rust para realizar _scans_ de portas TCP em endereços IPv4, permitindo tanto a varredura de um único IP quanto de um intervalo de endereços. Utiliza a biblioteca **Termion** para desenho de interface no terminal e **Chrono** para geração de timestamps em arquivos de log.

## 2. Dependências
No arquivo `Cargo.toml`, inclua:
```toml
[dependencies]
termion = "1.5"
chrono  = "0.4"
```
- **termion**: renderização de interface (cores, cursor, teclado)
- **chrono**: geração de timestamp para nome de logs

## 3. Estrutura do Projeto
- `src/main.rs`: arquivo único contendo toda a lógica da aplicação
- Logs gerados dinamicamente no diretório de execução, com nomes no formato `YYYYMMDD_HHMMSS.log`

## 4. Descrição das Funções e Módulos

### 4.1 Conversão de IP
```rust
fn ipv4_to_u32(ip: Ipv4Addr) -> u32
fn u32_to_ipv4(v: u32) -> Ipv4Addr
```
- **Propósito**: converter endereços IPv4 em valores `u32` e vice‑versa, facilitando a iteração sobre faixas de IP.
- **Uso interno**: essencial na opção de scan por intervalo de endereços.

### 4.2 Limpeza e Desenho de Interface
```rust
fn clear_screen()
fn draw_border() -> io::Result<()>
fn draw_menu() -> io::Result<()>
```
- `clear_screen()`: limpa todo o terminal e posiciona o cursor em `(1,1)`.
- `draw_border()`: desenha um contorno com **fundo azul** e **texto amarelo** ao redor do terminal.
- `draw_menu()`: invoca `clear_screen()` e `draw_border()`, centraliza o título "Brazucascan" e subtítulo, e lista as opções:
  1. Scanner por endereço (faixa de portas)
  2. Scanner por range de endereço
  3. Sair

### 4.3 Leitura de Entrada
```rust
fn read_line() -> io::Result<String>
```
- Lê linha do _stdin_, faz `trim()` e retorna o conteúdo como `String`.

### 4.4 Scanner por Endereço Único
```rust
fn scan_by_address() -> io::Result<()>
```
Fluxo:
1. Limpa tela e desenha interface.
2. Solicita:
   - **IPv4**
   - **Porta Inicial** (padrão 1)
   - **Porta Final**   (padrão = Porta Inicial)
3. Para cada porta no intervalo:
   - Atualiza barra de progresso (`[#####     ] XX%`).
   - Tenta conexão TCP com timeout de **100 ms**.
   - Se aberta, imprime `• Porta <n> aberta` em linha dedicada.
4. Ao fim, pergunta "Gerar .log? (y/n)":
   - **y**: cria arquivo `YYYYMMDD_HHMMSS.log` com o IP escaneado e portas abertas.
   - Qualquer outro: volta ao menu sem log.
5. Limpa tela e retorna ao menu.

### 4.5 Scanner por Intervalo de Endereços
```rust
fn scan_by_range_address() -> io::Result<()>
```
Fluxo semelhante ao scanner único, com as diferenças:
1. Solicita:
   - **IP Inicial**
   - **IP Final**
   - **Porta Inicial**
   - **Porta Final**
2. Converte IPs para `u32` e itera toda a faixa de endereços e portas.
3. Exibe barra de progresso e imprime `• <IP>:<porta> aberta` para cada porta acessível.
4. Pergunta sobre geração de log:
   - Registra faixa de IPs e portas abertas no arquivo `YYYYMMDD_HHMMSS.log`.
5. Aguarda que o usuário pressione **R** (ou **r**) para retornar ao menu.

### 4.6 Função Principal
```rust
fn main() -> io::Result<()> {
    loop {
        draw_menu()?;
        match read_line()?.as_str() {
            "1" => scan_by_address()?,
            "2" => scan_by_range_address()?,
            "3" => { /* limpa e sai */ break; },
            _   => { println!("Opção inválida"); let _ = read_line(); clear_screen(); }
        }
    }
    Ok(())
}
```
- Mantém o _loop_ até que o usuário escolha **3) Sair**.
- Tratamento de opção inválida reinicia o menu após tecla Enter.

## 5. Fluxo de Uso
1. No terminal, dentro da pasta do projeto:
   ```bash
   cargo build --release
   ./target/release/brazucascan
   ```
2. Escolha a opção desejada no menu.
3. Informe os dados solicitados.
4. Acompanhe o progresso e resultados em tempo real.
5. (Opcional) Gere o arquivo de log para análise posterior.

## 6. Logs
- Nome: `<YYYYMMDD>_<HHMMSS>.log`
- Conteúdo:
  - **IP escaneado** ou **faixa de IPs**
  - Listagem de portas ou IP:porta abertas

## 7. Sugestões de Melhoria
- **Timeout** configurável via argumento CLI.
- **Execução paralela** (threads, async) para acelerar a varredura.
- **Suporte IPv6**.
- Parâmetros de cor e estilo personalizáveis.

## 8. Autor
Desenvolvido por **Tony Garcia**. Qualquer dúvida ou sugestão, abra uma _issue_ no repositório ou contate diretamente.

---
*Fim da documentação*


