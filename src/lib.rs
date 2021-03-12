use sapp_jsutils::JsObject;

pub fn set_cookie(name: &str, data: &str) {
    unsafe { setCookie(JsObject::string(name), JsObject::string(data), 10); }
}

pub fn get_cookie(name: &str) -> String {
    let cstring = unsafe { getCookie(JsObject::string(name)) };
    let mut string = String::new();
    cstring.to_string(&mut string);
    return string;
}

extern "C" {

    fn setCookie(cname: JsObject, cvalue: JsObject, exdays: u32);

    fn getCookie(cname: JsObject) -> JsObject;

}