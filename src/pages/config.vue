<template>
    <div class="config-page">
        <div class="config-container">
            <h2>项目配置</h2>
            <p class="config-description">
                请填写以下配置信息来自定义您的项目。配置完成后，项目将自动适配您的GitHub仓库和域名。
            </p>
            
            <el-form :model="configForm" :rules="rules" ref="configFormRef" label-width="120px">
                <el-form-item label="GitHub用户名" prop="owner" required>
                    <el-input 
                        v-model="configForm.owner" 
                        placeholder="请输入您的GitHub用户名或组织名"
                        clearable
                    />
                    <div class="form-tip">例如：your-username</div>
                </el-form-item>
                
                <el-form-item label="仓库名称" prop="repo" required>
                    <el-input 
                        v-model="configForm.repo" 
                        placeholder="请输入您的仓库名称"
                        clearable
                    />
                    <div class="form-tip">例如：MyAwesomeApp</div>
                </el-form-item>
                
                <el-form-item label="自定义域名" prop="domain">
                    <el-input 
                        v-model="configForm.domain" 
                        placeholder="请输入您的自定义域名（可选）"
                        clearable
                    />
                    <div class="form-tip">例如：myapp.com（可选，留空将使用GitHub Pages默认域名）</div>
                </el-form-item>
                
                <el-form-item label="联系邮箱" prop="email">
                    <el-input 
                        v-model="configForm.email" 
                        placeholder="请输入您的联系邮箱（可选）"
                        clearable
                    />
                    <div class="form-tip">例如：your-email@example.com（可选）</div>
                </el-form-item>
                
                <el-form-item>
                    <el-button type="primary" @click="saveConfig" :loading="saving">
                        保存配置
                    </el-button>
                    <el-button @click="resetConfig">
                        重置
                    </el-button>
                    <el-button @click="previewUrls" type="info">
                        预览生成的URL
                    </el-button>
                </el-form-item>
            </el-form>
            
            <!-- 预览生成的URL -->
            <div v-if="showPreview" class="url-preview">
                <h3>生成的URL预览</h3>
                <div class="url-list">
                    <div class="url-item">
                        <label>GitHub仓库：</label>
                        <a :href="previewData.github" target="_blank">{{ previewData.github }}</a>
                    </div>
                    <div class="url-item">
                        <label>发布页面：</label>
                        <a :href="previewData.releases" target="_blank">{{ previewData.releases }}</a>
                    </div>
                    <div class="url-item">
                        <label>项目主页：</label>
                        <a :href="previewData.pages" target="_blank">{{ previewData.pages }}</a>
                    </div>
                    <div class="url-item">
                        <label>API地址：</label>
                        <span>{{ previewData.api }}</span>
                    </div>
                </div>
            </div>
            
            <!-- 当前配置状态 -->
            <div class="current-config">
                <h3>当前配置</h3>
                <div class="config-status">
                    <el-tag :type="isConfigured ? 'success' : 'warning'">
                        {{ isConfigured ? '已配置' : '未配置' }}
                    </el-tag>
                    <span v-if="isConfigured" class="config-info">
                        {{ currentConfig.owner }}/{{ currentConfig.repo }}
                    </span>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { repositoryConfig, generateGitHubUrls, reloadRepositoryConfig, type RepositoryConfig } from '@/config/repository'

const configFormRef = ref<FormInstance>()
const saving = ref(false)
const showPreview = ref(false)

// 表单数据
const configForm = reactive<RepositoryConfig>({
    owner: '',
    repo: '',
    domain: '',
    email: ''
})

// 当前配置
const currentConfig = ref<RepositoryConfig>({ ...repositoryConfig })

// 预览数据
const previewData = ref<any>({})

// 表单验证规则
const rules: FormRules = {
    owner: [
        { required: true, message: '请输入GitHub用户名', trigger: 'blur' },
        { min: 1, max: 39, message: '用户名长度应在1-39个字符之间', trigger: 'blur' },
        { pattern: /^[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?$/, message: '用户名格式不正确', trigger: 'blur' }
    ],
    repo: [
        { required: true, message: '请输入仓库名称', trigger: 'blur' },
        { min: 1, max: 100, message: '仓库名称长度应在1-100个字符之间', trigger: 'blur' },
        { pattern: /^[a-zA-Z0-9._-]+$/, message: '仓库名称只能包含字母、数字、点、下划线和连字符', trigger: 'blur' }
    ],
    domain: [
        { pattern: /^([a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?\.)+(com|org|net|edu|gov|mil|int|co|io|me|app|dev|tech|info|biz|name|pro|museum|aero|coop|jobs|travel|xxx|post|tel|asia|cat|mobi|xxx|arpa|[a-z]{2})$/, message: '请输入有效的域名', trigger: 'blur' }
    ],
    email: [
        { type: 'email', message: '请输入有效的邮箱地址', trigger: 'blur' }
    ]
}

// 计算属性：是否已配置
const isConfigured = computed(() => {
    return currentConfig.value.owner !== 'your-username' && 
           currentConfig.value.repo !== 'WebAppBuilder'
})

// 加载当前配置
const loadCurrentConfig = () => {
    // 从localStorage加载保存的配置
    const savedConfig = localStorage.getItem('pakePlus_config')
    if (savedConfig) {
        try {
            const parsed = JSON.parse(savedConfig)
            Object.assign(currentConfig.value, parsed)
            Object.assign(configForm, parsed)
        } catch (error) {
            console.error('加载配置失败:', error)
        }
    } else {
        // 使用默认配置
        Object.assign(configForm, currentConfig.value)
    }
}

// 保存配置
const saveConfig = async () => {
    if (!configFormRef.value) return
    
    try {
        await configFormRef.value.validate()
        saving.value = true
        
        // 保存到localStorage
        const configToSave = {
            owner: configForm.owner,
            repo: configForm.repo,
            domain: configForm.domain || '',
            email: configForm.email || ''
        }
        
        localStorage.setItem('pakePlus_config', JSON.stringify(configToSave))
        Object.assign(currentConfig.value, configToSave)
        
        // 重新加载配置
        reloadRepositoryConfig()
        
        ElMessage.success('配置保存成功！')
        
        // 提示用户重启应用
        await ElMessageBox.confirm(
            '配置已保存。为了使配置生效，建议重启应用。是否现在重启？',
            '配置保存成功',
            {
                confirmButtonText: '重启应用',
                cancelButtonText: '稍后重启',
                type: 'success'
            }
        )
        
        // 如果是Tauri环境，可以调用重启API
        if (window.__TAURI__) {
            const { relaunch } = await import('@tauri-apps/plugin-process')
            await relaunch()
        } else {
            // Web环境，刷新页面
            window.location.reload()
        }
        
    } catch (error) {
        if (error !== 'cancel') {
            console.error('保存配置失败:', error)
            ElMessage.error('保存配置失败')
        }
    } finally {
        saving.value = false
    }
}

// 重置配置
const resetConfig = () => {
    Object.assign(configForm, {
        owner: '',
        repo: '',
        domain: '',
        email: ''
    })
    showPreview.value = false
}

// 预览生成的URL
const previewUrls = () => {
    if (!configForm.owner || !configForm.repo) {
        ElMessage.warning('请先填写GitHub用户名和仓库名称')
        return
    }
    
    const tempConfig = {
        owner: configForm.owner,
        repo: configForm.repo,
        domain: configForm.domain,
        email: configForm.email
    }
    
    previewData.value = generateGitHubUrls(tempConfig)
    showPreview.value = true
}

onMounted(() => {
    loadCurrentConfig()
})
</script>

<style scoped>
.config-page {
    min-height: 100vh;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    padding: 20px;
    display: flex;
    justify-content: center;
    align-items: flex-start;
}

.config-container {
    background: white;
    border-radius: 12px;
    padding: 30px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
    max-width: 800px;
    width: 100%;
    margin-top: 20px;
}

h2 {
    color: #333;
    margin-bottom: 10px;
    text-align: center;
}

.config-description {
    color: #666;
    text-align: center;
    margin-bottom: 30px;
    line-height: 1.6;
}

.form-tip {
    font-size: 12px;
    color: #999;
    margin-top: 4px;
}

.url-preview {
    margin-top: 30px;
    padding: 20px;
    background: #f8f9fa;
    border-radius: 8px;
    border: 1px solid #e9ecef;
}

.url-preview h3 {
    margin-bottom: 15px;
    color: #333;
}

.url-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.url-item {
    display: flex;
    align-items: center;
    gap: 10px;
}

.url-item label {
    font-weight: 500;
    min-width: 80px;
    color: #555;
}

.url-item a {
    color: #409eff;
    text-decoration: none;
}

.url-item a:hover {
    text-decoration: underline;
}

.current-config {
    margin-top: 30px;
    padding: 20px;
    background: #f0f9ff;
    border-radius: 8px;
    border: 1px solid #bfdbfe;
}

.current-config h3 {
    margin-bottom: 15px;
    color: #333;
}

.config-status {
    display: flex;
    align-items: center;
    gap: 10px;
}

.config-info {
    font-family: 'Courier New', monospace;
    background: #e5e7eb;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 14px;
}

@media (max-width: 768px) {
    .config-container {
        margin: 10px;
        padding: 20px;
    }
    
    .url-item {
        flex-direction: column;
        align-items: flex-start;
    }
    
    .url-item label {
        min-width: auto;
    }
}
</style>