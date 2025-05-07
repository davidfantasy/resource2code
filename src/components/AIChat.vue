<template>
    <div class="code-generator-form">
        <el-card shadow="never" class="form-card">
            <template #header>
                <div class="card-header">
                    <span class="header-title">AI 代码生成</span>
                    <div class="header-actions">
                        <el-button type="primary" @click="submitForm" :loading="isTaskRunning">
                            执行任务
                        </el-button>
                        <el-button type="info" @click="showRuleManagerDialog" plain>
                            <el-icon>
                                <Setting />
                            </el-icon> 规则管理
                        </el-button>
                        <el-button type="info" @click="consoleVisible = true" plain>
                            <el-icon>
                                <Monitor />
                            </el-icon> 控制台
                        </el-button>
                    </div>
                </div>
            </template>

            <div class="scrollable-content">
                <el-form ref="formRef" :model="form" label-position="top" class="form-container">
                    <!-- 代码生成需求 -->
                    <el-form-item prop="requirement" class="form-section">
                        <template #label>
                            <div class="label-with-tooltip">
                                <span>代码生成提示词</span>
                                <el-tooltip effect="dark"
                                    content="代码生成任务的提示词，如有需要可以修改默认提示词，添加更具体的需求。代码生成相关的规范细节建议使用规则进行复用，不建议直接写到这里"
                                    placement="top">
                                    <el-icon class="tooltip-icon">
                                        <QuestionFilled />
                                    </el-icon>
                                </el-tooltip>
                            </div>
                        </template>
                        <el-input v-model="form.question" type="textarea" :rows="5"
                            placeholder="请详细描述您需要生成的代码功能、输入输出、业务逻辑等..." clearable class="requirement-input" />
                    </el-form-item>

                    <!-- 代码示例 -->
                    <el-form-item prop="selectedExamples" class="form-section">
                        <template #label>
                            <div class="label-with-tooltip">
                                <span>自定义规则</span>
                                <el-tooltip effect="dark" content="你可以在规则中自由定义您期望的代码风格和格式等" placement="top">
                                    <el-icon class="tooltip-icon">
                                        <QuestionFilled />
                                    </el-icon>
                                </el-tooltip>
                            </div>
                        </template>
                        <div class="code-examples-container">
                            <div v-if="rules.length === 0" class="empty-state">
                                <el-icon>
                                    <InfoFilled />
                                </el-icon>
                                <span>暂无自定义规则，请先添加</span>
                            </div>

                            <div v-else class="examples-checkbox-group">
                                <el-checkbox-group v-model="form.sampleIds">
                                    <div class="example-row" v-for="(row, rowIndex) in ruleRows" :key="rowIndex">
                                        <div v-for="rule in row" :key="rule.id" class="example-item">
                                            <el-checkbox :value="rule.id" class="example-checkbox">
                                                <div class="example-content">
                                                    <el-icon>
                                                        <Document />
                                                    </el-icon>
                                                    <span class="example-name" @click.stop="previewRule(rule)">{{
                                                        rule.name
                                                        }}</span>
                                                </div>
                                            </el-checkbox>
                                        </div>
                                    </div>
                                </el-checkbox-group>
                            </div>
                        </div>
                    </el-form-item>

                    <!-- 其它相关资源 -->
                    <el-form-item prop="resources" class="form-section">
                        <template #label>
                            <div class="label-with-tooltip">
                                <span>依赖资源</span>
                                <el-tooltip effect="dark" content="从右侧资源栏中添加代码生成所依赖的资源信息，比如数据表的schema或者某些代码文件"
                                    placement="top">
                                    <el-icon class="tooltip-icon">
                                        <QuestionFilled />
                                    </el-icon>
                                </el-tooltip>
                            </div>
                        </template>
                        <div class="resources-container">
                            <div v-if="resources.length === 0" class="empty-state">
                                <el-icon>
                                    <FolderOpened />
                                </el-icon>
                                <span>点击右侧资源管理器中的资源名称进行添加</span>
                            </div>
                            <div v-else class="resources-list">
                                <el-tag v-for="(resource, index) in resources" :key="index" class="resource-tag"
                                    closable :type="getResourceTagType(resource)" @close="removeResource(index)">
                                    <el-icon class="resource-icon">
                                        <component :is="getResourceIcon(resource)" />
                                    </el-icon>
                                    {{ resource.name }}
                                </el-tag>
                            </div>
                        </div>
                    </el-form-item>
                </el-form>
            </div>

            <!-- 固定底部选项 -->
            <div class="fixed-options">
                <el-form-item label="生成选项" class="options-section">
                    <div class="options-group">
                        <div class="label-with-tooltip">
                            <el-checkbox v-model="form.autoDetectDir" label="自动判断生成目录" size="large" />
                            <el-tooltip effect="dark" content="基于当前的项目目录结构让AI自动判断生成代码的所在的目录" placement="top">
                                <el-icon class="tooltip-icon">
                                    <QuestionFilled />
                                </el-icon>
                            </el-tooltip>
                        </div>
                    </div>
                </el-form-item>
            </div>
        </el-card>

        <!-- 代码示例管理对话框 -->
        <el-dialog @closed="loadRules" v-model="ruleManagerDialogVisible" title="代码示例管理" width="70%" top="5vh"
            class="example-manager-dialog">
            <RuleManagerDialog @close="ruleManagerDialogVisible = false" @refresh="loadRules" />
        </el-dialog>

        <!-- 生成结果控制台 -->
        <el-drawer v-model="consoleVisible" title="代码生成控制台" direction="btt" size="50%">
            <div class="console-container" ref="consoleContainer">
                <div v-for="(log, index) in consoleLogs" :key="index" class="log-entry">
                    <el-icon :class="`log-icon-${log.level.toLowerCase()}`">
                        <component :is="getLogIcon(log.level)" />
                    </el-icon>
                    <span class="log-message">{{ log.message }}</span>
                    <span class="log-timestamp">
                        {{ new Date(log.timestamp).toLocaleTimeString() }}
                    </span>
                </div>
            </div>
            <template #footer>
                <el-button @click="consoleVisible = false">关闭</el-button>
                <el-button v-if="isTaskRunning" type="danger" @click="cancelTask">取消</el-button>
            </template>
        </el-drawer>
    </div>
    <CodeResultViewer ref="resultViewerRef" />
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed, watch, nextTick, h } from 'vue'
import {
    Document,
    InfoFilled,
    FolderOpened,
    CircleCloseFilled,
    WarningFilled,
    Grid,
    Setting,
    Monitor,
} from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox, type FormInstance } from 'element-plus'
import RuleManagerDialog from './RuleManagerDialog.vue'
import { QuestionFilled } from '@element-plus/icons-vue'
import { Rule, ruleService } from '../services/RuleService'
import { invoke } from '@tauri-apps/api/core'
import { CodeFile, TaskLog, TaskLogLevel, TaskResult } from '../services/dto'
import CodeResultViewer from './CodeGenResultViewer.vue'
import { marked } from 'marked'


interface ResourceMeta {
    name: string
    resourceType: 'file' | 'table'
    data: string
}

const props = defineProps({
    resources: {
        type: Array as () => ResourceMeta[],
        default: () => []
    }
})

const emit = defineEmits(['resource-remove'])
const resultViewerRef = ref<InstanceType<typeof CodeResultViewer>>()

const formRef = ref<FormInstance>()
const form = reactive({
    question: '请基于提供的规则和资源，生成符合要求的代码文件',
    sampleIds: [] as string[],
    resources: [] as ResourceMeta[],
    currentSrcDir: '',
    autoDetectDir: true,
})

const rules = ref<Rule[]>([])
const ruleManagerDialogVisible = ref(false)
const consoleVisible = ref(false)
const consoleLogs = ref<TaskLog[]>([])
const isTaskRunning = ref(false)
const currentTaskId = ref<string | null>(null)

const ruleRows = computed(() => {
    const rows = []
    const itemsPerRow = 3
    for (let i = 0; i < rules.value.length; i += itemsPerRow) {
        rows.push(rules.value.slice(i, i + itemsPerRow))
    }
    return rows
})
const loadRules = async () => {
    try {
        ruleService.list().then(data => {
            rules.value = data
        })
    } catch (error) {
        console.error('加载规则失败:', error)
    }
}

const showRuleManagerDialog = () => {
    ruleManagerDialogVisible.value = true
}

const removeResource = (index: number) => {
    emit('resource-remove', index)
}

const getResourceIcon = (resource: ResourceMeta) => {
    return resource.resourceType === 'table' ? Grid : Document
}

const getResourceTagType = (resource: ResourceMeta) => {
    return resource.resourceType === 'table' ? 'success' : ''
}

const getLogIcon = (level: TaskLogLevel) => {
    switch (level) {
        case TaskLogLevel.Warn: return WarningFilled
        case TaskLogLevel.Error: return CircleCloseFilled
        default: return InfoFilled
    }
}
let isTaskChecking = false;

const submitForm = async () => {
    if (!form.question.trim()) {
        ElMessage.warning('请填写相关的代码生成提示词');
        return
    }
    form.resources = props.resources;
    form.currentSrcDir = await invoke('get_config', { key: "root_source_path" });
    consoleVisible.value = true
    isTaskRunning.value = true
    try {
        let taskId = await invoke<string>('process_user_question', { request: form });
        currentTaskId.value = taskId;
        // 设置定时器，每秒检查一次任务状态
        const intervalId = setInterval(async () => {
            try {
                if (isTaskChecking) {
                    return;
                }
                isTaskChecking = true;
                const finished = await invoke('is_user_task_finished', { taskId });
                const taskLogs = await invoke<TaskLog[]>('get_user_task_logs', { taskId });
                consoleLogs.value = taskLogs;
                if (finished) {
                    clearInterval(intervalId);
                    isTaskRunning.value = false;
                    currentTaskId.value = null;
                    const taskResult = await invoke<TaskResult>('get_user_task_result', { taskId });
                    console.log('任务结果:', taskResult);
                    resultViewerRef.value?.openDialog(taskResult.data.files as CodeFile[]);
                }
            } catch (error) {
                clearInterval(intervalId);
                console.error('获取任务状态失败:', error);
            } finally {
                isTaskChecking = false;
            }
        }, 200);
    } catch (error) {
        isTaskRunning.value = false
        consoleVisible.value = false
        ElMessage.error('代码生成失败:' + error)
    }
}

const cancelTask = async () => {
    let taskId = currentTaskId.value;
    try {
        await invoke('cancel_task', { taskId });
    } catch (error) {
        ElMessage.error('取消任务失败:' + error)
    }
    isTaskRunning.value = false
    currentTaskId.value = null
}

const previewRule = (rule: Rule) => {
    ElMessageBox({
        title: rule.name,
        message: h('div', {
            style: 'max-height: 70vh; overflow: auto;',
            innerHTML: marked.parse(rule.content || '暂无详细内容')
        }),
        showConfirmButton: false,
        closeOnClickModal: true
    })
}

onMounted(() => {
    loadRules()
})

// 监听 consoleLogs 的变化，自动滚动到最新的日志
const consoleContainer = ref<HTMLElement | null>(null);
watch(consoleLogs, () => {
    nextTick(() => {
        if (consoleContainer.value) {
            consoleContainer.value.scrollTop = consoleContainer.value.scrollHeight;
        }
    });
}, { deep: true });



</script>

<style lang="scss" scoped>
.code-generator-form {
    height: 100%;
    padding: 16px;
    background-color: var(--el-bg-color-page);
    display: flex;
    flex-direction: column;

    .form-card {
        flex: 1;
        display: flex;
        flex-direction: column;
        background-color: var(--el-bg-color);
        border: 1px solid var(--border-color);
        border-radius: 4px;
        overflow: hidden;

        :deep(.el-card__header) {
            background-color: var(--el-bg-color);
            border-bottom: 1px solid var(--border-color);
            padding: 12px 16px;
        }

        :deep(.el-card__body) {
            flex: 1;
            display: flex;
            flex-direction: column;
            padding: 0;
            overflow: hidden;
        }

        .card-header {
            display: flex;
            justify-content: space-between;
            align-items: center;

            .header-title {
                font-size: 16px;
                font-weight: 500;
                color: var(--el-text-color-primary);
            }

            .header-actions {
                display: flex;
                gap: 8px;
            }
        }

        .scrollable-content {
            flex: 1;
            overflow-y: auto;
            padding: 16px;
        }

        .fixed-options {
            padding: 16px;
            border-top: 1px solid var(--border-color);
            background-color: var(--el-bg-color);
        }
    }

    .form-section {
        margin-bottom: 20px;
        width: 100%;

        :deep(.el-form-item__label) {
            font-weight: 500;
            color: var(--el-text-color-primary);
            padding-bottom: 8px;
        }
    }

    .requirement-input {
        width: 100%;

        :deep(.el-textarea__inner) {
            background-color: var(--el-bg-color);
            border: 1px solid var(--border-color);
            color: var(--el-text-color-primary);
            font-family: var(--el-font-family);
            resize: vertical;
        }
    }

    .input-tips {
        font-size: 12px;
        color: var(--el-text-color-secondary);
        margin-top: 4px;
    }

    .code-examples-container,
    .resources-container {
        border: 1px solid var(--border-color);
        border-radius: 4px;
        padding: 12px;
        background-color: var(--el-fill-color-light);
        width: 100%;
    }

    .section-toolbar {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 12px;

        .section-label {
            font-size: 14px;
            font-weight: 500;
            color: var(--el-text-color-primary);
        }

        .section-tip {
            font-size: 12px;
            color: var(--el-text-color-secondary);
        }
    }

    .empty-state {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 16px;
        color: var(--el-text-color-secondary);
        font-size: 14px;
        background-color: var(--el-bg-color);
        border-radius: 4px;
        border: 1px dashed var(--border-color);

        .el-icon {
            margin-right: 8px;
            font-size: 16px;
        }
    }

    .examples-checkbox-group {
        overflow-y: auto;
        padding: 4px;
    }

    .example-row {
        display: flex;
        flex-wrap: wrap;
        margin-bottom: 8px;
        gap: 8px;
    }

    .example-item {
        flex: 1;
        min-width: 200px;
        transition: all 0.2s;

        &:hover {
            background-color: var(--el-fill-color-light);
            border-radius: 4px;
        }
    }

    .example-checkbox {
        width: 100%;
        padding: 8px;

        :deep(.el-checkbox__label) {
            width: 100%;
        }
    }

    .example-content {
        display: flex;
        align-items: center;
        gap: 8px;

        .example-name {
            flex: 1;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }

        .el-icon {
            color: var(--el-color-primary);
        }
    }

    .resources-list {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
    }

    .resource-tag {
        padding: 0 10px;
        height: 28px;
        line-height: 26px;

        .resource-icon {
            margin-right: 4px;
            font-size: 14px;
        }
    }

    .options-section {
        :deep(.el-form-item__label) {
            display: none;
        }
    }

    .options-group {
        display: flex;
        gap: 24px;
    }

    .console-container {
        padding: 16px;
        height: 100%;
        overflow-y: auto;
        background-color: var(--el-bg-color);
        font-family: monospace;
        font-size: 14px;

        .log-entry {
            display: flex;
            align-items: center;
            padding: 6px 0;
            border-bottom: 1px solid var(--border-color);

            .log-icon-info {
                color: var(--el-color-info);
                margin-right: 8px;
            }

            .log-icon-success {
                color: var(--el-color-success);
                margin-right: 8px;
            }

            .log-icon-warning {
                color: var(--el-color-warning);
                margin-right: 8px;
            }

            .log-icon-error {
                color: var(--el-color-error);
                margin-right: 8px;
            }

            .log-message {
                flex: 1;
                color: var(--el-text-color-primary);
            }

            .log-timestamp {
                color: var(--el-text-color-secondary);
                font-size: 12px;
                margin-left: 12px;
            }
        }
    }
}

.example-manager-dialog {
    :deep(.el-dialog__body) {
        padding: 0;
        height: calc(70vh - 110px);
        overflow: hidden;
    }
}

.label-with-tooltip {
    display: flex;
    align-items: center;
    gap: 6px;

    .tooltip-icon {
        color: var(--el-text-color-secondary);
        cursor: help;
        font-size: 14px;

        &:hover {
            color: var(--el-color-primary);
        }
    }
}
</style>