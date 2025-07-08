# 🌊 Surfing - 给终端下载装上冲浪板的艺术（Rust重制版）

<div class="attribution-notice" style="text-align:center; margin:20px 0; font-size:0.9em; color:#666;">
  <a href="https://dribbble.com/shots/1835728-Surf-s-up-dude"
     target="_blank"
     rel="noopener noreferrer"
     style="display:inline-block; text-decoration:none; color:#ea4c89;">
  </a>
</div>
<p align="center">
  <br>
  <a href="https://github.com/Geekstrange/Surfing"><img src="https://img.shields.io/badge/Version-0.0.1-cyan?style=for-the-badge&logo=rust"></a>
  <a href="https://github.com/Geekstrange/Surfing/blob/main/LICENSE"><img src="https://forthebadge.com/images/badges/cc-nc-sa.svg"></a>
  <a href="https://github.com/Geekstrange/Surfing/stargazers"><img src="https://img.shields.io/github/stars/Geekstrange/Surfing?color=yellow&style=for-the-badge&logo=github"></a>
</p>


> "从Bash到Rust，冲浪板升级为喷气式滑板！" —— 基于Surfing-curl的重构进化版

---

## 🚀 重构进化史

### ⚡ 技术升级亮点
- **跨平台引擎**：基于Rust重构，原生支持Windows/macOS/Linux
- **零依赖设计**：彻底摆脱curl工具链，自研下载核心
- **性能飞跃**：下载速度提升300%，内存占用减少50%
- **现代架构**：异步I/O + 多线程处理，轻松应对大文件下载

### 🔄 继承与创新
| 特性                | Surfing-curl (Bash) | Surfing (Rust)       |
|---------------------|---------------------|----------------------|
| 波浪动画            | ✅                  | ✅ 更流畅            |
| 经典进度条          | ✅                  | ✅ 更精确            |
| 跨平台支持          | ❌ Linux only       | ✅ 全平台            |
| 依赖要求            | ❌ 需curl/bc等工具  | ✅完全独立         |
| 安装包大小          | ~200KB              | ~5MB (静态链接)      |
| 最大文件支持        | 2GB                 | 无限制               |

---

## 🛠️ 安装指南


### Windows专属
```powershell
# PowerShell一键安装
iwr -useb https://github.com/Geekstrange/Surfing/releases/latest/download/surfing_amd64.exe -o $env:TEMP\surfing.exe; mv $env:TEMP\surfing.exe C:\Windows\System32\
```


### Linux专属
```bash
# 全系通用
wget https://github.com/Geekstrange/Surfing/releases/latest/download/surfing_amd64
```

---

## 🎮 使用指南

### 基础冲浪姿势（全平台一致）
```bash
surfing "https://example.com/large-file.zip" "download.zip"
```


### 专业技巧
```bash
# 批量下载模式
echo -e "https://site/file1\nhttps://site/file2" | surfing --batch

# 生成下载报告（JSON格式）
surfing "http://example.com/data" "dataset.bin" --report > download.json
```

---

## 版本选择指南

根据您的操作系统和架构，请选择对应的安装文件以下是不同平台的版本对应关系：

| **操作系统** | **架构**              | **文件名**                      |
| :----------- | :-------------------- | :------------------------------ |
| **Windows**  | x86_64/AMD64          | `surfing-vx.x.x-windows_amd64.exe` |
| **Linux**    | x86_64/AMD64          | `surfing-vx.x.x-linux_amd64`       |
|              | ARM64/AArch64         | `surfing-vx.x.x-linux_arm64`       |
| **macOS**    | Intel (x86_64/AMD64)        | `surfing-vx.x.x-darwin_amd64`      |
|              | Apple Silicon (ARM64/AArch64) | `surfing-vx.x.x-darwin_arm64`      |

## 如何确定我的系统架构

### Windows 系统

1. **打开命令提示符**：可以通过在开始菜单中搜索“cmd”或“命令提示符”来打开

2. **输入命令**：在命令提示符中输入以下命令并按回车键：

   ```cmd
   wmic os get osarchitecture
   ```

3. **查看输出结果**：

   - 如果显示“64-bit”，则您的系统是 **64位 (x86_64)**
   - 如果显示“ARM64”，则您的系统是 **ARM64**

### Linux 系统

1. **打开终端**：可以通过在应用程序菜单中搜索“终端”或使用快捷键（通常是`Ctrl+Alt+T`）来打开

2. **输入命令**：在终端中输入以下命令并按回车键：

   ```bash
   uname -m
   ```

3. **查看输出结果**：

   - 如果显示`x86_64`，则您的系统是 **64位 (x86_64)**
   - 如果显示`aarch64`，则您的系统是 **ARM64**

### macOS 系统

1. **打开终端**：可以通过在应用程序菜单中搜索“终端”来打开

2. **输入命令**：在终端中输入以下命令并按回车键：

   ```zsh
   uname -m
   ```

3. **查看输出结果**：

   - 如果显示`x86_64`，则您的系统是 **Intel (x86_64)**
   - 如果显示`arm64`，则您的系统是 **Apple Silicon (ARM64)**

---

## 📜 开源宪章
### 继承自Surfing-curl的协议精神
- **核心原则不变**：仍采用CC-BY-NC-SA 4.0协议
- **全新代码基础**：100% Rust重写，零Bash遗留代码
- **跨平台承诺**：保证所有功能在各平台一致实现

### 开发者福利
```bash
# 编译自定义版本
git clone https://github.com/Geekstrange/Surfing
cd Surfing

# 开启所有功能
cargo build --release --features "multi-thread reporting"

# 自定义波浪字符集
WAVE_CHARS="🌊 🏄‍♂️ 🏄‍♀️" cargo run -- "https://..." "file.bin"
```

---

<p align="center">
  © 2025 Geekstrange - 保留所有冲浪权利<br>
  <sub>
    本代码采用
    <a href="https://creativecommons.org/licenses/by-nc-sa/4.0/"
       target="_blank"
       style="color: #2F80ED; text-decoration: underline dotted;">
      CC-BY-NC-SA 4.0
    </a>
    许可协议<br>
    <span style="font-size:0.8em; color: #666;">
      基于Surfing-curl理念重构 · Rust强力驱动
    </span>
  </sub>
</p>

<p align="center">
  Made with ❤️ and 🦀 in Terminal World<br>
  「让每个下载都成为视觉SPA」
</p>
