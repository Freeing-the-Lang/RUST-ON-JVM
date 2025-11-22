pub fn rust_type_to_jvm(ty: &str) -> String {
    match ty {
        "i32" => "I".into(),
        "i64" => "J".into(),
        "u8"  => "B".into(),
        "bool" => "Z".into(),
        "()" => "V".into(),
        "String" => "Ljava/lang/String;".into(),
        t if t.starts_with("Vec<") => {
            let inner = &t[4..t.len()-1];
            format!("[{}", rust_type_to_jvm(inner))
        }
        t if t.starts_with("&") => {
            rust_type_to_jvm(&t[1..])
        }
        other => format!("Lrust/{};", other.replace("::", "/")),
    }
}

pub fn rust_fn_sig_to_jvm(params: &[String], ret: &str) -> String {
    let mut out = "(".to_string();
    for p in params {
        out.push_str(&rust_type_to_jvm(p));
    }
    out.push(')');
    out.push_str(&rust_type_to_jvm(ret));
    out
}
