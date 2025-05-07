<template>
    <el-dialog v-model="visible" title="LLM 配置" width="600px">
        <el-form label-width="120px">
            <el-form-item label="供应商">
                <el-select v-model="form.name" placeholder="请选择LLM供应商">
                    <el-option label="Ollama" value="Ollama" />
                    <el-option label="OpenAI 兼容接口" value="OpenAI" />
                </el-select>
            </el-form-item>

            <el-form-item label="模型名称">
                <el-input v-model="form.model" />
            </el-form-item>

            <el-form-item label="基础URL">
                <el-input v-model="form.baseUrl" />
            </el-form-item>

            <el-form-item label="API Key">
                <el-input v-model="form.apiKey" type="password" show-password />
            </el-form-item>

            <el-form-item label="最大Token数">
                <el-input-number v-model="form.maxTokens" :min="100" />
            </el-form-item>

        </el-form>

        <template #footer>
            <el-button @click="visible = false">取消</el-button>
            <el-button type="primary" @click="handleSubmit">保存</el-button>
        </template>
    </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'

interface LLMProvider {
    name: string;
    model: string;
    baseUrl: string;
    apiKey: string;
    maxTokens: number;
}

const visible = ref(false)
const llmProviders = ref<Record<string, LLMProvider>>({})
const form = ref<LLMProvider>({
    name: '',
    model: '',
    baseUrl: '',
    apiKey: '',
    maxTokens: 2048
})

// 加载配置数据
const loadConfig = async () => {
    try {
        // 加载所有供应商配置
        const providers = await invoke<string>('get_config', { key: "llm_providers" })
        if (providers) {
            llmProviders.value = JSON.parse(providers)
        }
        // 加载当前选择的供应商
        const currentProvider = await invoke<string>('get_config', { key: "current_llm_provider" })
        if (currentProvider) {
            form.value = JSON.parse(currentProvider)
        }
    } catch (error) {
        console.error('加载配置失败:', error)
    }
}

// 切换供应商时更新表单数据
watch(() => form.value.name, async (newName) => {
    if (newName && llmProviders.value[newName]) {
        form.value = { ...llmProviders.value[newName] }
    } else {
        form.value = {
            name: newName,
            model: '',
            baseUrl: '',
            apiKey: '',
            maxTokens: 2048
        }
    }
})

const handleSubmit = async () => {
    if (!form.value.name) {
        ElMessage.error('请选择供应商')
        return
    }
    if (!form.value.model) {
        ElMessage.error('请输入模型名称')
        return
    }
    if (!form.value.baseUrl) {
        ElMessage.error('请输入基础URL')
        return
    }
    try {
        new URL(form.value.baseUrl)
    } catch {
        ElMessage.error('请输入有效的URL地址')
        return
    }

    try {
        // 更新当前供应商配置
        const updatedProviders = { ...llmProviders.value }
        updatedProviders[form.value.name] = form.value

        // 保存配置
        await invoke('set_config', {
            key: "llm_providers",
            value: JSON.stringify(updatedProviders)
        })
        await invoke('set_config', {
            key: "current_llm_provider",
            value: JSON.stringify(form.value)
        })

        ElMessage.success('配置保存成功')
        visible.value = false
    } catch (error) {
        ElMessage.error('保存配置失败')
        console.error('保存配置失败:', error)
    }
}

defineExpose({
    open: async () => {
        await loadConfig()
        visible.value = true
    }
})
</script>

<style scoped>
.el-form-item {
    margin-bottom: 20px;
}
</style>