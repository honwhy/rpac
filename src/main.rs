use boa_engine::{Context, Source};
use std::path::Path;

fn main() {
    // PAC文件的内容
    let pac_code = r#"
        function isPlainHostName(host) {
            return (host.indexOf('.') == -1);
        }

        function shExpMatch(host, pattern) {
            // 这里应该是正则表达式匹配的实现，为了简单起见，这里省略实现
            // return true;
        }
        function FindProxyForURL(url, host) {
            if (isPlainHostName(host) || shExpMatch(host, "*.local")) {
                return "DIRECT";
            }
            if (shExpMatch(host, "*.example.com")) {
                return "PROXY proxy.example.com:8080";
            }
            return "HTTPS proxy.example.com:8080";
        }
    "#;

    // 实例化执行上下文
    let mut context = Context::default();

    // 加载PAC文件内容
    let source = Source::from_bytes(pac_code);

    // 执行PAC文件中的JavaScript代码
    match context.eval(source) {
        Ok(res) => {
            println!("PAC script loaded successfully.");
        }
        Err(e) => {
            eprintln!("Failed to load PAC script: {:?}", e);
        }
    }

    // 调用FindProxyForURL方法
    let url = "http://www.example.com";
    let host = "www.example.com";
    match context.eval(Source::from_bytes(&format!(
        "FindProxyForURL('{}', '{}')",
        url, host
    ))) {
        Ok(res) => {
            println!("Proxy for {}: {:?}", url, res.to_string(&mut context).unwrap());
        }
        Err(e) => {
            eprintln!("Error calling FindProxyForURL: {:?}", e);
        }
    }
}