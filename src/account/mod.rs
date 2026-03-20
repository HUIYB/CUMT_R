use http::my_dbg;
use self_save::Savable;
#[derive(Default, Debug)]
pub struct Account {
    user_account: String,
    user_password: String,
}
impl Account {
    const M: &[u8] = b"&$&$&$";
    pub fn get() -> Self {
        let mut reader = String::new();
        println!("请输入账号");
        std::io::stdin().read_line(&mut reader).unwrap();
        let user_account = my_dbg!(reader.clone());
        println!("请输入密码");
        reader.clear();
        std::io::stdin().read_line(&mut reader).unwrap();
        let user_password = my_dbg!(reader);
        Self {
            user_account: user_account.trim().to_string(),
            user_password: user_password.trim().to_string(),
        }
    }
    pub fn get_account(&self) -> &str {
        &self.user_account
    }
    pub fn get_password(&self) -> &str {
        &self.user_password
    }
}
impl Savable<Account> for Account {
    fn to_binary(&self) -> Vec<u8> {
        let account = self.user_account.as_bytes();
        let password = self.user_password.as_bytes();
        [account, b"&$&$&$", password].concat()
    }
    fn from_binary(data: &[u8]) -> Option<Account> {
        if let Some((account, password)) = split_marker(data, Self::M) {
            Some(Account {
                user_account: String::from_utf8_lossy(&account).into_owned(),
                user_password: String::from_utf8_lossy(&password).into_owned(),
            })
        } else {
            None
        }
    }
}
fn split_marker(data: &[u8], marker: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    data.windows(marker.len())
        .position(|w| w == marker)
        .map(|pos| (data[..pos].to_vec(), data[pos + marker.len()..].to_vec()))
}
