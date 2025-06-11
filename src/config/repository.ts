// 仓库配置文件
// 用户可以修改这里的配置来指向自己的仓库

export interface RepositoryConfig {
    owner: string // GitHub用户名或组织名
    repo: string // 仓库名称
    domain?: string // 自定义域名（可选）
    email?: string // 联系邮箱（可选）
}

// 默认配置
const defaultConfig: RepositoryConfig = {
    owner: 'your-username',
    repo: 'WebAppBuilder',
    domain: 'your-domain.com',
    email: 'your-email@example.com',
}

// 从localStorage获取配置，如果没有则使用默认配置
const getRepositoryConfig = (): RepositoryConfig => {
    try {
        const savedConfig = localStorage.getItem('pakePlus_config')
        if (savedConfig) {
            const parsed = JSON.parse(savedConfig)
            return {
                owner: parsed.owner || defaultConfig.owner,
                repo: parsed.repo || defaultConfig.repo,
                domain: parsed.domain || defaultConfig.domain,
                email: parsed.email || defaultConfig.email,
            }
        }
    } catch (error) {
        console.error('读取配置失败，使用默认配置:', error)
    }
    return { ...defaultConfig }
}

// 导出配置（动态获取）
export let repositoryConfig: RepositoryConfig = getRepositoryConfig()

// 重新加载配置的函数
export const reloadRepositoryConfig = (): RepositoryConfig => {
    repositoryConfig = getRepositoryConfig()
    return repositoryConfig
}

// 生成GitHub相关URL的辅助函数
export const generateGitHubUrls = (config: RepositoryConfig = repositoryConfig) => {
    const baseUrl = `https://github.com/${config.owner}/${config.repo}`
    const apiUrl = `https://api.github.com/repos/${config.owner}/${config.repo}`
    const rawUrl = `https://raw.githubusercontent.com/${config.owner}/${config.repo}/main`
    const pagesUrl = config.domain ? `https://${config.domain}` : `https://${config.owner}.github.io/${config.repo}`
    
    return {
        github: baseUrl,
        releases: `${baseUrl}/releases`,
        api: apiUrl,
        raw: rawUrl,
        pages: pagesUrl,
        updaterEndpoint: `${apiUrl}/releases/latest`,
        updateJson: `${pagesUrl}/update.json`
    }
}

// 导出生成的URL
export const gitHubUrls = generateGitHubUrls()