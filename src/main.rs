use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Result, Write},
    path::Path,
};

/// BOM を追加したいファイルの入れ場所
const ROOT_DIRECTORY: &str = "./files";

/// UTF-8 バイト順マーク（BOM）
/// Reference: <https://ja.wikipedia.org/wiki/%E3%83%90%E3%82%A4%E3%83%88%E9%A0%86%E3%83%9E%E3%83%BC%E3%82%AF#%E5%90%84%E7%AC%A6%E5%8F%B7%E5%8C%96%E5%BD%A2%E5%BC%8F%EF%BC%88%E7%AC%A6%E5%8F%B7%E5%8C%96%E3%82%B9%E3%82%AD%E3%83%BC%E3%83%A0%EF%BC%89%E3%81%94%E3%81%A8%E3%81%AE%E3%83%90%E3%82%A4%E3%83%88%E9%A0%86%E3%83%9E%E3%83%BC%E3%82%AF>
/// Rust ではバイト値が十六進数である。
const BOM: &[u8] = b"\xEF\xBB\xBF";

fn main() -> Result<()> {
    let root = Path::new(ROOT_DIRECTORY);
    traverse_directory(root)?;
    wait_for_read_log()?;
    Ok(())
}

/// ディレクトリを走査する。
fn traverse_directory(directory: &Path) -> Result<()> {
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();

        // Dotfile を濾過する
        if path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("")
            .starts_with('.')
        {
            continue;
        }

        // ディレクトリの場合は更に潜る
        if path.is_dir() {
            traverse_directory(&path)?;
            continue;
        }

        // ファイルに BOM を追加する
        add_bom_to_file(&path)?;
    }

    Ok(())
}

/// ファイルに BOM を追加する。
fn add_bom_to_file(path: &Path) -> Result<()> {
    // ファイルの中身をバッファーに読み込む
    let mut original_content = Vec::<u8>::new();
    File::open(path)?.read_to_end(&mut original_content)?;

    // BOM が既に存在した場合は処理は不要
    let unix_style_path = path.to_str().unwrap_or("").replace("\\", "/");
    if original_content.starts_with(BOM) {
        println!("[{}] にBOM は既に存在しています。", unix_style_path);
        return Ok(());
    }

    // ファイルの中身を BOM 付きで新しいバッファーに読み込む
    let mut new_content = Vec::<u8>::with_capacity(BOM.len() + original_content.len());
    new_content.extend_from_slice(BOM);
    new_content.extend_from_slice(&original_content);

    // 新しいバッファーの中身をファイルに書き込む
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?
        .write_all(&new_content)?;

    println!("[{}] に BOM を追加しました。", unix_style_path);
    Ok(())
}

fn wait_for_read_log() -> Result<()> {
    println!("任意の鍵を押下して閉じる...");
    io::stdin().read_line(&mut String::new())?;
    Ok(())
}
