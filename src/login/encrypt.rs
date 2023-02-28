use aes::{Aes128, cipher::{KeyIvInit, BlockEncryptMut, block_padding::Pkcs7}};
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;
use regex::Regex;

const CHARSET: &str = "ABCDEFGHJKMNPQRSTWXYZabcdefhijkmnprstwxyz2345678";

fn rand_str(len: i32) -> String {
    let mut res = String::new();
    let charset: Vec<char> = CHARSET.chars().collect();
    let charset_length = charset.len();

    for _i in 0..len {
        let index = rand::thread_rng().gen_range(0..charset_length);
        res += &charset[index as usize].to_string();
    }

    res
}

fn gas<'a>(data: &'a str, key: &'a str, iv: &'a str) -> String {
    let reg = Regex::new(r"(^\s+)|(\s+$)").unwrap();
    let key = reg.replace_all(key, "");

    let cipher_text = cbc::Encryptor::<Aes128>::new_from_slices(key.as_bytes(), iv.as_bytes())
        .unwrap()
        .encrypt_padded_vec_mut::<Pkcs7>(data.as_bytes());

    general_purpose::STANDARD.encode(cipher_text)
}

/// 统一身份认证 AES-CBC 加密函数
pub fn encrypt_aes_cbc<'a>(data: &'a str, p1: &'a str) -> String {
    if p1.len() == 0 {
        return data.to_owned();
    }

    let data = rand_str(64) + data;
    let iv = rand_str(16);
    gas(&data, p1, &iv)
}