<template>
    <div class="vscode-container">
        <el-container class="main-container">
            <!-- 顶部 Header 区域 -->
            <el-header class="app-header">
                <div class="header-left">
                    <img class="app-icon" width="32" height="32" src="/logo.png">
                    <span class="app-title">Resource To Code</span>
                </div>
                <div class="header-right">
                    <el-button size="small" @click="llmConfigDialog?.open()">
                        <el-icon>
                            <Setting />
                        </el-icon>
                        <span>LLM 配置</span>
                    </el-button>
                </div>
            </el-header>

            <el-container>
                <!-- 左侧 Aside 区域 -->
                <el-aside :width="asideWidth + 'px'" class="app-aside" ref="asideRef">
                    <div class="aside-content">
                        <resource-explorer @resource-add="handleResourceAdd" />
                    </div>
                    <div class="resize-handle" @mousedown="startResize"></div>
                </el-aside>
                <!-- 右侧 Main 区域 -->
                <el-main class="app-main">
                    <div class="main-content">
                        <AIChat :resources="resources" @resource-remove="handleResourceRemove"></AIChat>
                    </div>
                </el-main>
            </el-container>
        </el-container>
    </div>
    <LLMConfigDialog ref="llmConfigDialog" />
</template>

<script setup lang="ts">
import {
    ElementPlus,
    Setting,
} from '@element-plus/icons-vue'
import ResourceExplorer from './components/ResourceExplorer.vue'
import AIChat from './components/AIChat.vue'
import LLMConfigDialog from './components/LLMConfigDialog.vue'
import { ResourceMeta } from './services/dto'
import { ref, onUnmounted } from 'vue'

const llmConfigDialog = ref<InstanceType<typeof LLMConfigDialog>>()
const resources = ref<ResourceMeta[]>([])
const asideWidth = ref(250)
const isResizing = ref(false)
const asideRef = ref<HTMLElement | null>(null)

const handleResourceAdd = (resource: ResourceMeta) => {
    resources.value.push(resource)
}
const handleResourceRemove = (index: number) => {
    resources.value.splice(index, 1)
}

const startResize = (e: MouseEvent) => {
    isResizing.value = true
    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseup', stopResize)
    e.preventDefault()
}

const handleMouseMove = (e: MouseEvent) => {
    if (!isResizing.value) return
    const newWidth = e.clientX
    if (newWidth > 200 && newWidth < 500) { // 限制最小200px，最大500px
        asideWidth.value = newWidth
    }
}

const stopResize = () => {
    isResizing.value = false
    document.removeEventListener('mousemove', handleMouseMove)
    document.removeEventListener('mouseup', stopResize)
}

onUnmounted(() => {
    document.removeEventListener('mousemove', handleMouseMove)
    document.removeEventListener('mouseup', stopResize)
})
</script>

<style setup lang="scss">
@import './styles/main.scss';
</style>