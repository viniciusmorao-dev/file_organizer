# Organizador de Arquivos (Rust)

Utilitario em Rust para organizacao automatizada de arquivos entre diretorios locais (`~/Downloads` para `~/Documents` e `~/Pictures`), com consumo inferior a 2 MB de memoria RAM.

---

## Regras de Organizacao

1. **Documentos** -> `~/Documents/`
   - Extensoes: `.pdf`, `.docx`, `.doc`, `.txt`, `.odt`, `.xlsx`, `.xls`, `.pptx`, `.ppt`, `.csv`, `.rtf`, `.epub`.
2. **Imagens** -> `~/Pictures/`
   - Extensoes: `.jpg`, `.jpeg`, `.png`, `.gif`, `.webp`, `.bmp`, `.svg`, `.tiff`, `.ico`.
3. **Outros formatos** (ex: `.zip`, `.mp4`, `.iso`):
   - Permanecem inalterados no diretorio de origem.
4. **Downloads parciais**:
   - Extensoes temporarias (`.crdownload`, `.part`, `.tmp`, `.aria2`) sao ignoradas ate a conclusao da gravacao.
5. **Tratamento de colisoes**:
   - Arquivos com nomes duplicados no destino recebem sufixo incremental numerico (`arquivo_1.ext`).

---

## Execucao

Binario de release compilado em:
`./target/x86_64-unknown-linux-musl/release/file_organizer`

### 1. Simulacao (Dry-Run)
Verifica o plano de movimentacao sem alterar o sistema de arquivos:
```bash
./target/x86_64-unknown-linux-musl/release/file_organizer --dry-run
```

### 2. Execucao Unica
Processa o diretorio de origem e finaliza:
```bash
./target/x86_64-unknown-linux-musl/release/file_organizer
```

### 3. Monitoramento Continuo
Executa em loop com intervalo configuravel (padrao: 3 segundos):
```bash
./target/x86_64-unknown-linux-musl/release/file_organizer --watch
```

---

## Configuracao com Systemd (Segundo Plano)

Para execucao como servico em nivel de usuario no Linux:

1. Instalar a unidade de servico:
   ```bash
   mkdir -p ~/.config/systemd/user
   cp file-organizer.service ~/.config/systemd/user/
   ```

2. Carregar e iniciar o servico:
   ```bash
   systemctl --user daemon-reload
   systemctl --user enable --now file-organizer.service
   ```

3. Gerenciamento:
   ```bash
   systemctl --user status file-organizer.service
   systemctl --user stop file-organizer.service
   ```

---

## Testes Automatizados

Execucao da suite de testes de integracao:
```bash
source "$HOME/.cargo/env"
cargo test
```
