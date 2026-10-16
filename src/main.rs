mod config;
mod organizer;

use config::Config;
use organizer::organize_directory;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

fn print_help() {
    println!(
        r#"Organizador de Arquivos (Rust)

USO:
    file_organizer [OPCOES]

OPCOES:
    -w, --watch             Executa continuamente em segundo plano monitorando o diretorio
    -n, --dry-run           Apenas simula a organizacao sem mover arquivos
    -i, --interval <SECS>   Intervalo de verificacao no modo watch (padrao: 3 segundos)
    -s, --source <PATH>     Diretorio de origem (padrao: ~/Downloads)
        --docs <PATH>       Diretorio de documentos (padrao: ~/Documents)
        --pics <PATH>       Diretorio de imagens (padrao: ~/Pictures)
    -h, --help              Exibe esta mensagem de ajuda

EXEMPLOS:
    file_organizer --dry-run
    file_organizer
    file_organizer --watch
"#
    );
}

fn main() {
    let mut config = Config::default_from_env();
    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-n" | "--dry-run" => {
                config.dry_run = true;
            }
            "-w" | "--watch" => {
                config.watch = true;
            }
            "-i" | "--interval" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<u64>() {
                        config.interval_secs = val.max(1);
                    }
                    i += 1;
                }
            }
            "-s" | "--source" => {
                if i + 1 < args.len() {
                    config.source_dir = PathBuf::from(&args[i + 1]);
                    i += 1;
                }
            }
            "--docs" => {
                if i + 1 < args.len() {
                    config.docs_dir = PathBuf::from(&args[i + 1]);
                    i += 1;
                }
            }
            "--pics" => {
                if i + 1 < args.len() {
                    config.pics_dir = PathBuf::from(&args[i + 1]);
                    i += 1;
                }
            }
            unknown => {
                eprintln!("Aviso: Opcao desconhecida '{}'. Use --help.", unknown);
            }
        }
        i += 1;
    }

    println!("==================================================");
    println!("        Organizador de Arquivos (Rust)           ");
    println!("==================================================");
    println!(" Origem:     {}", config.source_dir.display());
    println!(" Documentos: {}", config.docs_dir.display());
    println!(" Imagens:    {}", config.pics_dir.display());
    println!(
        " Modo:       {}{}",
        if config.watch { "Monitoramento continuo (--watch)" } else { "Execucao unica" },
        if config.dry_run { " [SIMULACAO / DRY-RUN]" } else { "" }
    );
    println!("==================================================");

    if config.watch {
        println!(
            "Iniciando monitoramento a cada {}s. Pressione Ctrl+C para encerrar.\n",
            config.interval_secs
        );
        loop {
            if let Err(err) = organize_directory(&config) {
                eprintln!("Erro ao organizar: {}", err);
            }
            let _ = io::stdout().flush();
            sleep(Duration::from_secs(config.interval_secs));
        }
    } else {
        match organize_directory(&config) {
            Ok(count) => {
                if count == 0 {
                    println!("Nenhum arquivo elegivel para movimentacao foi encontrado.");
                } else {
                    println!(
                        "\nConcluido: total de arquivos {}: {}.",
                        if config.dry_run { "simulados" } else { "movidos" },
                        count
                    );
                }
            }
            Err(err) => {
                eprintln!("Erro durante a execucao: {}", err);
                std::process::exit(1);
            }
        }
    }
}
