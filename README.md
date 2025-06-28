# AI生成 慎用
# Cloudflare DDNS Rust

一个用 Rust 编写的 Cloudflare 动态 DNS 更新工具。

## 功能特性

- 支持 IPv4 和 IPv6 地址更新
- 自动检测和更新 DNS 记录
- 支持子域名和根域名
- 多个IP获取服务，自动故障转移
- 随机时间间隔检查 IP 变化（1秒到5分钟）
- 智能负载均衡，避免 API 请求集中
- 使用 rustls 替代 openssl，更安全轻量
- 详细的日志记录
- 灵活的配置方式（配置文件 + 环境变量）

## 配置

### 配置文件

复制 `config.json.example` 为 `config.json` 并填入你的配置：

```json
{
  "domain": "subdomain.your-domain.com",
  "root_domain": "your-domain.com",
  "ipv4": true,
  "ipv6": false,
  "token": "your_cloudflare_api_token",
}
```

### 环境变量

你也可以使用环境变量来配置（优先级高于配置文件）：

```bash
export DOMAIN="subdomain.your-domain.com"
export ROOT_DOMAIN="your-domain.com"
export IPV4=true
export IPV6=false
export TOKEN="your_cloudflare_api_token"
```

### 配置说明

- `domain`: 要更新的完整域名（如 subdomain.example.com 或 example.com）
- `root_domain`: 根域名，用于获取 Cloudflare Zone ID（如 example.com）
- `ipv4`: 是否启用 IPv4 更新
- `ipv6`: 是否启用 IPv6 更新
- `token`: Cloudflare API Token（推荐使用，更安全）

## 获取 Cloudflare API Token

1. 登录 [Cloudflare Dashboard](https://dash.cloudflare.com/)
2. 点击右上角的用户图标，选择 "My Profile"
3. 切换到 "API Tokens" 标签页
4. 点击 "Create Token" 按钮
5. 选择 "Custom token" 模板
6. 配置权限：
   - **Zone** - `Zone:Read`
   - **Zone** - `DNS:Edit`
7. 在 "Zone Resources" 中选择你要管理的域名
8. 点击 "Continue to summary" 然后 "Create Token"
9. 复制生成的 Token（这是唯一一次显示完整 Token）

> **注意**: API Token 比 Global API Key 更安全，因为它可以限制权限范围和访问的资源。

## 运行

### 开发模式

```bash
cargo run
```

### 发布模式

```bash
cargo build --release
./target/release/cf-ddns-rust
```

## 日志

程序使用 `env_logger` 进行日志记录。你可以通过设置 `RUST_LOG` 环境变量来控制日志级别：

```bash
# 显示所有日志
RUST_LOG=debug cargo run

# 只显示错误和警告
RUST_LOG=warn cargo run
```

## IP 获取服务

为了提高稳定性，程序使用多个 IP 获取服务，并实现自动故障转移机制：

### IPv4 服务列表
1. **api.ipify.org** - 主要服务（JSON 格式）
2. **ipinfo.io** - 备用服务（纯文本）
3. **icanhazip.com** - 备用服务（纯文本）
4. **checkip.amazonaws.com** - AWS 服务（纯文本）

### IPv6 服务列表
1. **api64.ipify.org** - 主要服务（JSON 格式）
2. **ipv6.icanhazip.com** - 备用服务（纯文本）
3. **v6.ident.me** - 备用服务（纯文本）

### 故障转移机制
- 按顺序尝试每个服务
- 如果某个服务失败，自动切换到下一个
- 支持 JSON 和纯文本两种响应格式
- 详细的日志记录，便于故障排查
- 只有所有服务都失败时才报错

## 工作原理

1. 程序启动时加载配置
2. 验证 Cloudflare 凭据并获取 Zone ID
3. 随机间隔（1秒到5分钟）检查当前公网 IP
4. 如果启用了 IPv4/IPv6，获取相应的 IP 地址
5. 调用 Cloudflare API 更新或创建 DNS 记录
6. 记录操作结果到日志
7. 等待随机时间后重复检查

## 依赖项

- `tokio`: 异步运行时
- `reqwest`: HTTP 客户端（使用 rustls-tls 后端）
- `serde`: 序列化/反序列化
- `anyhow`: 错误处理
- `config`: 配置管理
- `log` + `env_logger`: 日志记录
- `rand`: 随机数生成

**注意**: 本项目使用 `rustls` 作为 TLS 后端，不依赖 OpenSSL，提供更好的安全性和跨平台兼容性。

## 预编译二进制文件

项目使用 GitHub Actions 自动构建多平台二进制文件，支持以下目标：

### Linux 平台
- **x86_64 (amd64)**
  - GNU 版本：`cf-ddns-rust-x86_64-unknown-linux-gnu.tar.gz`
  - MUSL 版本：`cf-ddns-rust-x86_64-unknown-linux-musl.tar.gz`
- **aarch64 (arm64)**
  - GNU 版本：`cf-ddns-rust-aarch64-unknown-linux-gnu.tar.gz`
  - MUSL 版本：`cf-ddns-rust-aarch64-unknown-linux-musl.tar.gz`

### 下载和使用

1. 前往 [Releases 页面](https://github.com/your-username/cf-ddns-rust/releases) 下载最新版本
2. 选择适合你系统的二进制文件：
   - **GNU 版本**：适用于大多数 Linux 发行版（需要 glibc）
   - **MUSL 版本**：静态链接，无需额外依赖，适用于容器和嵌入式环境
3. 解压并运行：
   ```bash
   tar xzf cf-ddns-rust-*.tar.gz
   chmod +x cf-ddns-rust
   ./cf-ddns-rust
   ```

### 自动构建

项目配置了 GitHub Actions 工作流，在以下情况下自动构建：
- 推送新的版本标签（如 `v1.0.0`）
- 手动触发工作流

每次构建都会生成校验和文件 `checksums.txt`，可用于验证下载文件的完整性。

## 许可证

MIT License