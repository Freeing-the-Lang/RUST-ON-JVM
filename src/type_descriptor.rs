pub fn rust_fn_sig_to_jvm(params: &[&str], ret: &str) -> String {
    let mut desc = String::from("(");

    for p in params {
        desc.push_str(&rust_type_to_jvm(p));
    }

    desc.push(')');
    desc.push_str(&rust_type_to_jvm(ret));

    desc
}

pub fn rust_type_to_jvm(ty: &str) -> String {
    match ty {
        "i32" => "I".into(),
        "i64" => "J".into(),
        "u8"  => "B".into(),
        "bool" => "Z".into(),
        "()" => "V".into(),
        "String" => "Ljava/lang/String;".into(),

        // 리스트, 벡터 타입 매핑
        t if t.starts_with("Vec<") => {
            let inner = &t[4..t.len()-1];
            format!("[{}", rust_type_to_jvm(inner))
        }

        // 참조 타입 (&T)
        t if t.starts_with("&") => {
            let inner = &t[1..];
            rust_type_to_jvm(inner)
        }

        // 기타는 object 취급
        other => format!("Lrust/{};", other.replace("::", "/")),
    }
}
