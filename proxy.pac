function FindProxyForURL(url, host) {
    // 直接连接到本地地址
    if (isPlainHostName(host) || shExpMatch(host, "*.local")) {
        return "DIRECT";
    }

    // 通过代理服务器连接到特定的域名
    if (shExpMatch(host, "*.example.com")) {
        return "PROXY proxy.example.com:8080";
    }

    // 对于所有其他请求，使用HTTPS代理
    return "HTTPS proxy.example.com:8080";
}