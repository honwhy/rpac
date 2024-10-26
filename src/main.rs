use boa_engine::{Context, Source};
use std::path::Path;
use std::fs;

fn main() {
    // 实例化执行上下文
    let mut context = Context::default();

    // 获取当前文件的路径
    let current_dir = std::env::current_dir().unwrap();
    let js_file_path = current_dir.join("pac_utils.js");
    let pac_file_path = current_dir.join("proxy.pac");

    // 读取JavaScript文件内容
    let js_content = fs::read_to_string(js_file_path).expect("Unable to read JS file");

    // 读取PAC文件内容
    let pac_content = fs::read_to_string(pac_file_path).expect("Unable to read PAC file");
    // 加载PAC文件内容
    let _ = context.eval(Source::from_bytes(&js_content));

    // 执行PAC文件中的JavaScript代码
    match context.eval(Source::from_bytes(&pac_content)) {
        Ok(res) => {
            println!("PAC script loaded successfully.");
        }
        Err(e) => {
            eprintln!("Failed to load PAC script: {:?}", e);
        }
    }
    let _ = context.eval(Source::from_bytes("proxy.pac"));
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
    let url = "http://localhost:8080";
    let host = "localhost";
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