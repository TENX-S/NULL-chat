


use std::{
    sync::Arc,
    net::{
        IpAddr,
        Ipv4Addr,
    },
};
use rand_key::{
    RandKey,
    ToRandKey,
};



enum Auth {
    Root,
    Normal,
    Blocked,
}


pub struct User {

    name: Arc<String>,
    key: Arc<RandKey>,
    auth: Auth,
    ip: IpAddr,

}

impl User {

    pub fn new(name: &str, key: &str, ip: &str) -> Self {

        User {
            name: Arc::new(name.into()),
            key: Arc::new(key.to_randkey()),
            auth: Auth::Normal,
            ip: ip.parse::<IpAddr>().unwrap(),
        }

    }

    pub fn set_name(&mut self, val: &str) {
        self.name = Arc::new(val.into());
    }

    pub fn set_key(&mut self, val: &str) {
        self.key = Arc::new(val.to_randkey());
    }

    pub fn block(&mut self) {
        self.auth = Auth::Blocked;
    }

    pub fn name(&self) -> Arc<String> {
        Arc::clone(&self.name)
    }


}


impl Default for User {
    fn default() -> Self {
        User {
            name: Arc::new(String::default()),
            key: Arc::new(RandKey::default()),
            auth: Auth::Normal,
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        }
    }
}
