use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
    Nonce, // Or `Aes128Gcm`
};
use std::collections::HashMap;
use std::path::Path;
use windows::Win32::{Security::Cryptography::CRYPTOAPI_BLOB};

pub fn get_raw_cookies(host_url: &str) -> HashMap<String, Vec<u8>> {
    let app_data_path = std::env::var("LOCALAPPDATA").unwrap();
    // println!("{app_data_path}");
    let mut cookies_db_path =
        format!(r"{app_data_path}\Google\Chrome\User Data\Default\Network\Cookies");
    if !Path::new(&cookies_db_path).exists() {
        cookies_db_path = format!(r"{app_data_path}\Google\Chrome\User Data\Default\Cookies");
    } else if !Path::new(&cookies_db_path).exists() {
        cookies_db_path = format!(r"{app_data_path}\Google\Chrome Dev\User Data\Default\Cookies");
    } else if !Path::new(&cookies_db_path).exists() {
        panic!("Cookies 未发现,你确定安装了Chrome浏览器?");
    }

    let conn = rusqlite::Connection::open(cookies_db_path).unwrap();
    let mut stmt = conn
        .prepare("SELECT name,encrypted_value FROM cookies where host_key = ?")
        .unwrap();
    let mut rows = stmt.query(rusqlite::params![host_url]).unwrap();
    let mut data: HashMap<String, Vec<u8>> = HashMap::new();

    while let Some(row) = rows.next().unwrap() {
        let encrypted_value_ref = row.get_ref_unwrap(1);
        let encrypted_value = encrypted_value_ref.as_bytes().unwrap();
        data.insert(row.get(0).unwrap(), encrypted_value.to_vec());
    }
    data
}
pub fn get_key() -> Vec<u8> {
    let app_data_path = std::env::var("LOCALAPPDATA").unwrap();
    // println!("{app_data_path}");
    let mut local_state_path = format!(r"{app_data_path}\Google\Chrome\User Data\Local State");
    if !Path::new(&local_state_path).exists() {
        local_state_path = format!(r"{app_data_path}\Google\Chrome Dev\User Data\Local State");
    } else if !Path::new(&local_state_path).exists() {
        panic!("Local State 未发现,你确定安装了Chrome浏览器?");
    }
    // println!("{Local_State_path}");
    let f = std::fs::File::open(local_state_path).unwrap();
    let json: serde_json::Value = serde_json::from_reader(f).unwrap();
    // println!("{json:?}");
    let encrypted_key = json
        .get("os_crypt")
        .unwrap()
        .get("encrypted_key")
        .unwrap()
        .as_str()
        .unwrap();
    // println!("{encrypted_key:?}");
    let encrypted_key_bytes = base64::decode(encrypted_key).unwrap();

    let encrypted_key_bytes = &encrypted_key_bytes[5..];
    // println!("{:?}", encrypted_key);
    crypt_unprotect_data(encrypted_key_bytes).to_vec()
}

pub fn decrypt_string(key: &[u8], data: &[u8]) -> String {
    let iv = &data[3..15];
    let cipherbytes = &data[15..];

    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(iv); // 96-bits; unique per message
    let plaintext = cipher.decrypt(nonce, cipherbytes).unwrap();
    String::from_utf8(plaintext).unwrap()
}
fn crypt_unprotect_data(data: &[u8]) -> &[u8] {
    let mut out = CRYPTOAPI_BLOB::default();
    let mut data_vec = data.to_vec();
    let _rst = unsafe {
        let size = u32::try_from(data.len()).unwrap();
        let p_data_in = CRYPTOAPI_BLOB {
            cbData: size,
            pbData: data_vec.as_mut_ptr(),
        };
        windows::Win32::Security::Cryptography::CryptUnprotectData(
            &p_data_in,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            0,
            &mut out,
        )
    };
    let decode = unsafe {
        let output = core::slice::from_raw_parts(out.pbData, out.cbData as _);
        // Cleanup
        windows::Win32::System::Memory::LocalFree(out.pbData as _);
        output
    };
    decode
}
