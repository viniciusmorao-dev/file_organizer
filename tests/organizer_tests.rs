use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn create_temp_dir(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(name);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("Falha ao criar diretorio temporario para teste");
    path
}

#[test]
fn test_organizer_end_to_end() {
    let test_root = create_temp_dir("file_organizer_e2e_test");
    let downloads = test_root.join("Downloads");
    let docs = test_root.join("Documents");
    let pics = test_root.join("Pictures");

    fs::create_dir_all(&downloads).unwrap();
    fs::create_dir_all(&docs).unwrap();
    fs::create_dir_all(&pics).unwrap();

    File::create(downloads.join("relatorio.pdf")).unwrap().write_all(b"PDF").unwrap();
    File::create(downloads.join("anotacoes.txt")).unwrap().write_all(b"TXT").unwrap();
    File::create(downloads.join("artigo.docx")).unwrap().write_all(b"DOCX").unwrap();

    File::create(downloads.join("foto.png")).unwrap().write_all(b"PNG").unwrap();
    File::create(downloads.join("wallpaper.jpg")).unwrap().write_all(b"JPG").unwrap();

    File::create(downloads.join("instalador.zip")).unwrap().write_all(b"ZIP").unwrap();
    File::create(downloads.join("video.mp4")).unwrap().write_all(b"MP4").unwrap();

    File::create(downloads.join("grande_arquivo.pdf.crdownload")).unwrap().write_all(b"CRDOWNLOAD").unwrap();

    File::create(docs.join("relatorio.pdf")).unwrap().write_all(b"OLD_PDF").unwrap();

    let bin_path = PathBuf::from(env!("CARGO_BIN_EXE_file_organizer"));
    let status = std::process::Command::new(bin_path)
        .arg("--source")
        .arg(&downloads)
        .arg("--docs")
        .arg(&docs)
        .arg("--pics")
        .arg(&pics)
        .status()
        .expect("Falha ao executar o binario de teste");

    assert!(status.success());

    assert!(docs.join("relatorio_1.pdf").exists());
    assert!(docs.join("anotacoes.txt").exists());
    assert!(docs.join("artigo.docx").exists());

    assert!(pics.join("foto.png").exists());
    assert!(pics.join("wallpaper.jpg").exists());

    assert!(downloads.join("instalador.zip").exists());
    assert!(downloads.join("video.mp4").exists());
    assert!(downloads.join("grande_arquivo.pdf.crdownload").exists());

    let _ = fs::remove_dir_all(&test_root);
}
