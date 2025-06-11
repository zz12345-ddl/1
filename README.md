# WebAppBuilder

<span href=".README.md">English</span>
| <a href="README_ZH.md">简体中文</a>
| <a href="README_JP.md">日本語</a>

<p align="center">
  <img src="https://gw.alipayobjects.com/zos/k/fa/logo-modified.png" alt="WebAppBuilder" width="120" height="120" />
</p>

<h1 align="center">WebAppBuilder</h1>
<div align="center">
  <a href="https://github.com/your-username/WebAppBuilder/releases" target="_blank">
    <img alt="GitHub downloads" src="https://img.shields.io/github/downloads/your-username/WebAppBuilder/total.svg?style=flat-square" />
  </a>
  <a href="https://github.com/your-username/WebAppBuilder/releases" target="_blank">
    <img alt="GitHub release" src="https://img.shields.io/github/release/your-username/WebAppBuilder.svg?style=flat-square" />
  </a>
</div>

<div align="center">Turn any webpage into a desktop app with Rust. 🤱</div>

<div align="left">WebAppBuilder supports Mac, Windows, Linux, and Android & iOS. There's no need to install complex dependencies locally— Github Token is optional for basic usage. Plus, WebAppBuilder is only about 10MB in size. For the latest version, please see the <a href="https://github.com/your-username/WebAppBuilder/releases">release page</a>. Check the README for information on <a href="#popular-packages">popular packages</a> and <a href="#development">custom development</a>.</div>

https://github.com/user-attachments/assets/b88bf541-0b26-4020-9eec-da79e1734fc9

## Features

-   🎐 Approximately 20 times smaller than Electron (less than 5MB!) and 10 times faster.
-   🚀 Built with Rust Tauri, PakePlus is lighter and faster than JavaScript-based frameworks.
-   📦 Comes with rich built-in features — supports shortcuts, immersive windows, and minimalist customization.
-   👻 PakePlus is a minimalist tool that replaces traditional packaging with Tauri for cross-platform desktop apps.
-   📲 Uses native frameworks to package Android and iOS apps — smaller and faster than Tauri2.
-   🤗 Easy to use — just one GitHub Token is all you need to get a desktop app.
-   🌹 No need to install complex dependencies locally — use GitHub Actions for cloud-based automatic packaging.
-   🧑‍🤝‍🧑 Internationalization support — automatically follows your system language.
-   💡 Supports custom JavaScript injection — write your own JS to inject into the page.
-   🎨 Beautiful and user-friendly UI — better experience for beginners, supports Chinese names for packaging.
-   📡 Usable directly via the web, but the client offers stronger functionality and is recommended.
-   🔐 Data security — your token is stored only locally, and your project stays safely in your own Git repo.
-   🍀 Supports static file packaging — drop in a compiled dist folder or index.html from Vue/React to create a client app.
-   🐞 Debug mode supported — find and eliminate bugs during preview or release.
-   💬 If you run into any issues, feel free to join our technical community for help.

## Use Cases

-   Have a website? Instantly turn it into a app and elevate its appearance.
-   Have a Vue/React project and don’t want to buy a server? Package it as a desktop app.
-   Want your Cocos game to run as a cross-platform client? No problem.
-   Need to build your Unity project as a cross-platform client? Also no problem.
-   Hide your website address from casual sharing or bots scraping your content.
-   For internal company platforms — restrict access to your site via a dedicated client only.
-   Turn any website into your custom client — inject JS for automation and custom features.
-   Annoyed by website ads? Hide them with powerful JS.
-   Want to use Tauri2 but the environment is too heavy? Use PakePlus instead!

## Popular Packages

<img src=https://sjj1024.github.io/PakePlus/static/imgs/preview.webp  width=1920/>

PakePlus supports installation packages for both ARM and Intel architectures. Most popular program installation packages only include the ARM architecture for macOS and the Intel architecture for Windows. If you need installation packages for additional architectures, please use PakePlus to compile the required package separately.

<table>
    <tr>
        <td>DeepSeek
            <a href="https://github.com/Sjj1024/PakePlus/releases/download/OtherFiles/DeepSeek_0.0.1_aarch64.dmg">Mac</a>
            <a href="https://github.com/Sjj1024/PakePlus/releases/download/OtherFiles/DeepSeek_0.0.1_x64-setup.exe">Windows</a>
            <a href="https://github.com/Sjj1024/PakePlus/releases/download/OtherFiles/DeepSeek_0.0.1_amd64.deb">Linux</a>
        </td>
        <td>X (Twitter)
            <a href="https://github.com/codegirle/PakePlus/releases/download/Twitter/Twitter_0.0.1_aarch64.dmg">Mac</a>
            <a href="https://github.com/codegirle/PakePlus/releases/download/Twitter/Twitter_0.0.1_x64-setup.exe">Windows</a>
            <a href="https://github.com/codegirle/PakePlus/releases/download/Twitter/twitter_0.0.1_amd64.deb">Linux</a>
        </td>
    </tr>
    <tr>
        <td><img src=https://sjj1024.github.io/PakePlus/static/imgs/deepseek.png width=600/></td>
        <td><img src=https://sjj1024.github.io/PakePlus/static/imgs/xtwitter.png width=600/></td>
    </tr>
    <tr>
        <td>YouTube
            <a href="https://github.com/codegirle/PakePlus/releases/download/YouTube/YouTube_0.0.2_aarch64.dmg">Mac</a>
            <a href="https://github.com/codegirle/PakePlus/releases/download/YouTube/YouTube_0.0.2_x64-setup.exe">Windows</a>
            <a href="https://github.com/codegirle/PakePlus/releases/download/YouTube/you-tube_0.0.2_amd64.deb">Linux</a>
        </td>
        <td>小红书
            <a href="https://github.com/Sjj1024/PakePlus/releases/download/OtherFiles/_0.0.1_aarch64.dmg">Mac</a>
            <a href="https://github.com/Sjj1024/PakePlus/releases/download/OtherFiles/_0.0.1_x64-setup.exe">Windows</a>
            <a href="https://github.com/Sjj1024/PakePlus/releases/download/OtherFiles/_0.0.1_amd64.deb">Linux</a>
        </td>
    </tr>
    <tr>
        <td><img src=https://sjj1024.github.io/PakePlus/static/imgs/youtube.png width=600/></td>
        <td><img src=https://sjj1024.github.io/PakePlus/static/imgs/hongshu.png width=600/></td>
    </tr>
        <tr>
        <td>Tiktok
            <a href="https://github.com/codegirle/PakePlus/releases/download/Tiktok/Tiktok_0.0.2_aarch64.dmg">Mac</a>
            <a href="https://github.com/codegirle/PakePlus/releases/download/Tiktok/Tiktok_0.0.2_x64-setup.exe">Windows</a>
            <a href="https://github.com/codegirle/PakePlus/releases/download/Tiktok/tiktok_0.0.2_amd64.deb">Linux</a>
        </td>
        <td>句乐部
            <a href="https://gh-proxy.com/github.com/cuixiaorui/PakePlus/releases/download/juleu/julebu_1.0.0_aarch64.dmg">Mac</a>
            <a href="https://gh-proxy.com/github.com/cuixiaorui/PakePlus/releases/download/juleu/julebu_1.0.0_x64-setup.exe">Windows</a>
            <a href="https://gh-proxy.com/github.com/cuixiaorui/PakePlus/releases/download/juleu/julebu_1.0.0_amd64.deb">Linux</a>
        </td>
    </tr>
    <tr>
        <td><img src=https://sjj1024.github.io/PakePlus/static/imgs/tiktok.png width=600/></td>
        <td><img src=https://files.pakeplus.com/julebu.webp width=600/></td>
    </tr>
</table>

[![Powered by DartNode](https://dartnode.com/branding/DN-Open-Source-sm.png)](https://dartnode.com 'Powered by DartNode - Free VPS for Open Source')

## Getting Started

1. Download the app from the release page：https://github.com/Sjj1024/PakePlus/releases,  
   double-click to install, and run the app. or visit web: <a href="https://pakeplus.pages.dev" target="_blank">PakePlus Web</a>

2. Configure a Github Token, create a new project, and set up the configurations. get token guide:  
   <a href="https://sjj1024.github.io/PakePlus/index_en.html">English</a> <a href="https://pakeplus.com/guide/token.html" style="margin: 0 50px;">简体中文</a><a href="https://sjj1024.github.io/PakePlus/index_ja.html">日本语</a>

```
Explanation of Github Token permissions:
1. For the beta version, token permissions required:
All repositories: Fork an original template repository
Actions: Manage GitHub actions
Administration: Fork and manage files in the repository
Contents: Add, delete, modify, and find repository contents
Issues: Submit issues to PakePlus

1. For the classic version, token permissions required:
repo: Fork and manage template code
workflow: Compile and release your software
```

1. You can preview the app in a new window and click the publish button to package the app.
2. You can download the app from the release page.

## FAQ

1. Mac users may see an “App is damaged” warning on installation. click cancel and Run the following command, then reopen the app:（This is because the application requires an official signature to avoid the prompt of "The application is damaged" popping up after installation. However, the cost of the official signature is $99 per year... Therefore, it is necessary to manually bypass the signature to use it normally）

```sh
sudo xattr -r -d com.apple.quarantine /Applications/PakePlus.app
```

1. When you package the app, Mac users may see an “App is damaged” warning on installation. click cancel and Run the following command, then reopen the app:

```sh
sudo xattr -r -d com.apple.quarantine /Applications/YourAppName.app
```

3. If you encounter any issues while using the program, please download the latest version, as versions developed before 2025 were based on Tauri v1.0. In 2025, Tauri was upgraded to v2.0. If the latest version still has issues, please submit an issue with detailed information so that we can update and resolve it more quickly.

4. After you add the GitHub Token, PakePlus will fork a PakePlus repository into your own repository. All your future projects will rely on this repository for creation and compilation, so please do not delete your PakePlus repository.

## Developing PakePlus

If you want to develop PakePlus locally, ensure your environment is prepared beforehand. Make sure Rust `>=1.63` and Node `>=16` (e.g., `16.18.1`) are installed on your computer. For installation guidance, please refer to the [Tauri Documentation](https://tauri.app/v1/guides/getting-started/prerequisites).

If you’re unfamiliar with these, it’s best to try the one-click packaging tool above.

```sh
# Install dependencies
pnpm i

# Local development [Right-click to open debug mode.]
pnpm run dev

# Package the app
pnpm run build

```

## Support

1. wechat and group，my wechat is lanxingme，welcome to join the wechat group.
 <p align="center" style="display:flex; justify-content: flex-start;" >
    <img src="https://github.com/Sjj1024/PakePlus/raw/main/docs/static/imgs/mywx.png" width=200/>
    <img src="https://github.com/Sjj1024/PakePlus/raw/main/docs/static/imgs/wxcode.png" width=200/>
    <img src="https://github.com/Sjj1024/PakePlus/raw/main/docs/static/imgs/qq.jpg" width=200/>
 </p>

2. 💖If PakePlus has brought joy to your life, feel free to support it.
 <p align="center" style="display:flex; justify-content: flex-start;">
    <img src="https://github.com/Sjj1024/PakePlus/raw/main/docs/static/imgs/sponsor.webp" width=620/>
 </p>

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=Sjj1024/PakePlus,Sjj1024/PakePlus-iOS,Sjj1024/PakePlus-Android&type=Date)](https://www.star-history.com/#Sjj1024/PakePlus&Sjj1024/PakePlus-iOS&Sjj1024/PakePlus-Android&Date)
