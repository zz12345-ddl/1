// 仓库配置文件
// 用户可以修改这里的配置来指向自己的仓库

export interface RepositoryConfig {
    owner: string // GitHub用户名或组织名
    repo: string // 仓库名称
    domain?: string // 自定义域名（可选）
    email?: string // 联系邮箱（可选）
}

// 默认配置 - 用户需要修改这里的信息
export const repositoryConfig: RepositoryConfig = {
    owner: 'your-username', // 请修改为你的GitHub用户名
    repo: 'WebAppBuilder', // 请修改为你的仓库名称
    domain: 'your-domain.com', // 请修改为你的域名（可选）
    email: 'your-email@example.com', // 请修改为你的邮箱（可选）
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