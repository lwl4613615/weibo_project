use crypto::{digest::Digest, md5::Md5};

pub fn md5(valuse :&str,salt:&str) ->String{
    let mut hash=Md5::new();
    let str=format!("{}{}",valuse,salt);
    hash.input_str(&str);
    hash.result_str()
    

}