<template>
    <el-dialog v-model="visible" title="代码生成结果" width="80%" :before-close="handleClose" class="result-viewer-dialog">
        <el-table :data="files" border style="width: 100%" height="60vh">
            <el-table-column prop="name" label="文件名" width="180">
                <template #default="{ row }">
                    <div class="file-name">
                        <el-icon>
                            <Document />
                        </el-icon>
                        <span class="name-text">{{ row.name }}</span>
                    </div>
                </template>
            </el-table-column>
            <el-table-column label="文件路径" min-width="300">
                <template #default="{ row }">
                    <div class="path-editor">
                        <el-tooltip :content="row.path" placement="top" :disabled="!isPathTooLong(row.path)">
                            <el-input v-model="row.path" placeholder="文件路径" :disabled="row.applied" class="path-input"
                                :title="row.path" />
                        </el-tooltip>
                        <el-button type="primary" plain @click="selectFolder(row)" :icon="FolderOpened"
                            :disabled="row.applied" class="select-btn">
                            选择
                        </el-button>
                    </div>
                </template>
            </el-table-column>
            <el-table-column label="状态" width="120" align="center">
                <template #default="{ row }">
                    <el-tag :type="row.applied ? 'success' : 'info'" effect="light">
                        {{ row.applied ? '已应用' : '待应用' }}
                    </el-tag>
                </template>
            </el-table-column>
            <el-table-column label="操作" width="120" align="center">
                <template #default="{ row }">
                    <el-button type="primary" size="small" @click="applyFile(row)" v-if="!row.applied"
                        class="apply-btn">
                        应用
                    </el-button>
                    <span v-else class="applied-text">✓ 已应用</span>
                </template>
            </el-table-column>
        </el-table>
    </el-dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { FolderOpened, Document } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { CodeFile } from '../services/dto'
import { exists } from '@tauri-apps/plugin-fs';

const visible = ref(false)
const files = ref<CodeFile[]>([])

const isPathTooLong = (path: string) => {
    return path.length <= 40
}

const openDialog = (resultFiles: CodeFile[]) => {
    files.value = resultFiles.map(file => ({ ...file, applied: false }))
    visible.value = true
}

const selectFolder = async (file: CodeFile) => {
    try {
        let rootDir = await invoke<string>('get_config', { key: "root_source_path" })
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: file.path ? file.path : rootDir
        })
        if (selected && !Array.isArray(selected)) {
            file.path = selected + "/" + file.name
        }
    } catch (error) {
        ElMessage.error('选择目录失败: ' + error)
    }
}

const applyFile = async (file: CodeFile) => {
    try {
        let fileExisted = await invoke<boolean>("is_file_exsited", { filePath: file.path });
        if (fileExisted) {
            try {
                await ElMessageBox.confirm(
                    `文件 ${file.name} 已存在，是否要覆盖？`,
                    '警告',
                    {
                        confirmButtonText: '覆盖',
                        cancelButtonText: '取消',
                        type: 'warning',
                    }
                )
            } catch (cancel) {
                return; // 用户点击取消，直接返回不执行覆盖
            }
        }
        await invoke('save_generated_file', { file })
        file.applied = true
        ElMessage.success(`文件 ${file.name} 应用成功`)
    } catch (error) {
        ElMessage.error(`文件应用失败: ${error}`)
    }
}

const handleClose = (done: () => void) => {
    done()
}

defineExpose({
    openDialog
})
</script>

<style scoped>
.result-viewer-dialog {
    :deep(.el-dialog__body) {
        padding: 10px 20px;
    }
}

.path-editor {
    display: flex;
    gap: 8px;
    align-items: center;

    .path-input {
        flex: 1;

        :deep(.el-input__inner) {
            text-overflow: ellipsis;
            white-space: nowrap;
            overflow: hidden;
        }
    }

    .select-btn {
        flex-shrink: 0;
    }
}

.file-name {
    display: flex;
    align-items: center;
    gap: 8px;

    .el-icon {
        color: var(--el-color-primary);
    }

    .name-text {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }
}

.apply-btn {
    width: 80px;
}

.applied-text {
    color: var(--el-color-success);
    font-size: 13px;
}

:deep(.el-table) {
    .el-table__cell {
        padding: 12px 0;
    }

    .el-table__body tr:hover>td {
        background-color: var(--el-fill-color-light);
    }
}
</style>