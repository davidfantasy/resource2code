<template>
    <div class="code-example-manager">
        <el-container class="manager-container">
            <!-- 工具栏 -->
            <el-header class="toolbar">
                <el-button type="primary" @click="showAddDialog">
                    <el-icon>
                        <Plus />
                    </el-icon>添加示例
                </el-button>
                <el-input v-model="searchQuery" placeholder="搜索示例..." clearable
                    style="width: 300px; margin-left: 12px;">
                    <template #prefix>
                        <el-icon>
                            <Search />
                        </el-icon>
                    </template>
                </el-input>
            </el-header>

            <!-- 主内容区 -->
            <el-main class="main-content">
                <el-table :data="filteredExamples" border height="100%" style="width: 100%" empty-text="暂无代码示例">
                    <el-table-column prop="name" label="示例名称" width="180" />
                    <el-table-column prop="content" label="示例内容" show-overflow-tooltip />
                    <el-table-column label="操作" width="150" align="center">
                        <template #default="{ row, $index }">
                            <el-button size="small" @click="showEditDialog(row, $index)">
                                <el-icon>
                                    <Edit />
                                </el-icon>
                            </el-button>
                            <el-button size="small" type="danger" @click="confirmDelete(row.id, $index)">
                                <el-icon>
                                    <Delete />
                                </el-icon>
                            </el-button>
                        </template>
                    </el-table-column>
                </el-table>
            </el-main>
        </el-container>

        <!-- 添加/编辑对话框 -->
        <el-dialog v-model="exampleDialogVisible" title="编辑代码示例" width="60%">
            <el-form :model="exampleForm" label-position="top" ref="exampleFormRef">
                <el-form-item label="示例名称" prop="name" :rules="[{ required: true, message: '请输入示例名称' }]">
                    <el-input v-model="exampleForm.name" placeholder="输入示例名称" />
                </el-form-item>
                <el-form-item label="示例内容" prop="content" :rules="[{ required: true, message: '请输入示例内容' }]">
                    <el-input v-model="exampleForm.content" type="textarea" :rows="10" placeholder="输入代码示例内容" />
                </el-form-item>
            </el-form>
            <template #footer>
                <el-button @click="exampleDialogVisible = false">取消</el-button>
                <el-button type="primary" @click="saveExample">保存</el-button>
            </template>
        </el-dialog>
    </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { Plus, Search, Edit, Delete } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox, type FormInstance } from 'element-plus'
import { ruleService } from '../services/RuleService'

interface CodeExample {
    id: string
    name: string
    language?: string
    content: string
}

const emit = defineEmits(['close', 'refresh'])

const examples = ref<CodeExample[]>([])

const searchQuery = ref('')
const exampleDialogVisible = ref(false)
const currentExampleIndex = ref<number | null>(null)
const exampleFormRef = ref<FormInstance>()
const exampleForm = reactive({
    id: '',
    name: '',
    language: '',
    content: ''
})

const filteredExamples = computed(() => {
    if (!searchQuery.value) return examples.value;
    return examples.value.filter(example => {
        return example.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
            (example.language && example.language.toLowerCase().includes(searchQuery.value.toLowerCase()));
    });
});

const showAddDialog = () => {
    currentExampleIndex.value = null
    exampleForm.id = ''
    exampleForm.name = ''
    exampleForm.language = ''
    exampleForm.content = ''
    exampleDialogVisible.value = true
}

const showEditDialog = (example: CodeExample, index: number) => {
    currentExampleIndex.value = index
    exampleForm.id = example.id
    exampleForm.name = example.name
    exampleForm.language = example.language || ''
    exampleForm.content = example.content
    exampleDialogVisible.value = true
}

const saveExample = async () => {
    try {
        await exampleFormRef.value?.validate()

        const newExample = {
            id: exampleForm.id || Date.now().toString(),
            name: exampleForm.name,
            content: exampleForm.content
        }

        if (currentExampleIndex.value !== null) {
            ruleService.update(newExample)
            examples.value[currentExampleIndex.value] = newExample
        } else {
            ruleService.create(newExample)
            examples.value.push(newExample)
        }

        exampleDialogVisible.value = false
        emit('refresh')
    } catch (error) {
        console.error('验证失败:', error)
    }
}

const confirmDelete = (id: string, index: number) => {
    ElMessageBox.confirm('确定删除此代码示例吗?', '提示', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
    }).then(() => {
        ruleService.delete(id)
        examples.value.splice(index, 1)
        emit('refresh')
        ElMessage.success('删除成功')
    }).catch(() => { })
}

const loadCodeExamples = async () => {
    try {
        examples.value = await ruleService.list();
    } catch (error) {
        console.error('加载代码示例失败:', error)
    }
}

onMounted(() => {
    loadCodeExamples()
})
</script>

<style lang="scss" scoped>
.code-example-manager {
    height: 100%;
    display: flex;
    flex-direction: column;

    .manager-container {
        height: 100%;
    }

    .toolbar {
        display: flex;
        align-items: center;
        padding: 12px 16px;
        border-bottom: 1px solid var(--border-color);
        background-color: var(--el-bg-color);
    }

    .main-content {
        padding: 0;
        height: 100%;
    }
}
</style>