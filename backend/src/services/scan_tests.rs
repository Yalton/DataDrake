use super::scan::ScanService;
use base64::{engine::general_purpose, Engine as _};
use std::{fs, path::PathBuf};

#[tokio::test]
async fn scan_records_encoded_paths_and_file_sizes() {
    let temp_root = std::env::temp_dir().join(format!("data-drake-scan-{}", uuid::Uuid::new_v4()));
    let scan_root = temp_root.join("root");
    fs::create_dir_all(&scan_root).unwrap();
    fs::write(scan_root.join("sample.txt"), b"hello").unwrap();

    // scan_directory writes scans.log in its working directory, so keep that
    // side effect inside this test's temporary directory.
    let previous_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_root).unwrap();
    let scans = ScanService::new()
        .scan_directory(PathBuf::from(&scan_root))
        .await;
    std::env::set_current_dir(previous_dir).unwrap();

    let encoded_root = general_purpose::STANDARD.encode(scan_root.to_string_lossy().as_bytes());
    let file_path = scan_root.join("sample.txt");
    let encoded_file = general_purpose::STANDARD.encode(file_path.to_string_lossy().as_bytes());
    let file_scan = scans.iter().find(|scan| scan.path == encoded_file).unwrap();

    assert_eq!(file_scan.size, 5);
    assert!(scans.iter().all(|scan| scan.root_path == encoded_root));
    assert!(temp_root.join("scans.log").is_file());

    fs::remove_dir_all(temp_root).unwrap();
}
