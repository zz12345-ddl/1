#!/usr/bin/env node

import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

// 从 repository.ts 读取配置
const repositoryConfigPath = path.join(__dirname, '../src/config/repository.ts')
const repositoryContent = fs.readFileSync(repositoryConfigPath, 'utf8')

// 解析配置
const ownerMatch = repositoryContent.match(/owner:\s*['"]([^'"]+)['"]/)
const repoMatch = repositoryContent.match(/repo:\s*['"]([^'"]+)['"]/)
const domainMatch = repositoryContent.match(/domain:\s*['"]([^'"]+)['"]/)
const emailMatch = repositoryContent.match(/email:\s*['"]([^'"]+)['"]/)

if (!ownerMatch || !repoMatch) {
    console.error('无法从 repository.ts 中解析配置信息')
    process.exit(1)
}

const config = {
    owner: ownerMatch[1],
    repo: repoMatch[1],
    domain: domainMatch ? domainMatch[1] : '',
    email: emailMatch ? emailMatch[1] : ''
}

console.log('配置信息:', config)

// 替换 tauri.conf.json 中的模板变量
const tauriConfigPath = path.join(__dirname, '../src-tauri/tauri.conf.json')
let tauriConfig = fs.readFileSync(tauriConfigPath, 'utf8')

tauriConfig = tauriConfig
    .replace(/{{PRODUCT_NAME}}/g, config.repo)
    .replace(/{{PRODUCT_NAME_LOWER}}/g, config.repo.toLowerCase())
    .replace(/{{GITHUB_OWNER}}/g, config.owner)
    .replace(/{{REPO_NAME}}/g, config.repo)

fs.writeFileSync(tauriConfigPath, tauriConfig)

// 替换 package.json 中的名称
const packageJsonPath = path.join(__dirname, '../package.json')
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'))
packageJson.name = config.repo.toLowerCase()
fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2))

// 替换 Cargo.toml 中的名称
const cargoTomlPath = path.join(__dirname, '../src-tauri/Cargo.toml')
let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8')
cargoToml = cargoToml
    .replace(/name = "WebAppBuilder"/g, `name = "${config.repo}"`)
    .replace(/name = "webappbuilder_lib"/g, `name = "${config.repo.toLowerCase()}_lib"`)
fs.writeFileSync(cargoTomlPath, cargoToml)

// 替换 main.rs 中的引用
const mainRsPath = path.join(__dirname, '../src-tauri/src/main.rs')
let mainRs = fs.readFileSync(mainRsPath, 'utf8')
mainRs = mainRs.replace(/webappbuilder_lib::run\(\)/g, `${config.repo.toLowerCase()}_lib::run()`)
fs.writeFileSync(mainRsPath, mainRs)

// 更新 cmds.rs
const cmdsRsPath = path.join(__dirname, '../src-tauri/src/command/cmds.rs')
let cmdsRs = fs.readFileSync(cmdsRsPath, 'utf8')
cmdsRs = cmdsRs.replace(/\{\{GITHUB_OWNER\}\}/g, config.owner)
cmdsRs = cmdsRs.replace(/\{\{GITHUB_REPO\}\}/g, config.repo)
fs.writeFileSync(cmdsRsPath, cmdsRs)

console.log('配置完成！')
console.log(`产品名称: ${config.repo}`)
console.log(`GitHub 所有者: ${config.owner}`)
console.log(`GitHub 仓库: ${config.repo}`)