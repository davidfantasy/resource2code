<template>
    <el-dialog v-model="visible" :title="formData.id ? '编辑数据源' : '新建数据源'" width="600px" @close="handleClose">
        <el-form ref="formRef" :model="formData" :rules="rules" label-width="100px" label-position="left">
            <el-form-item label="名称" prop="name">
                <el-input v-model="formData.name" />
            </el-form-item>

            <el-form-item label="数据库类型" prop="dbType">
                <el-select v-model="formData.dbType">
                    <el-option label="MySQL" value="mysql" />
                    <el-option label="PostgreSQL" value="postgres" />
                    <el-option label="SQLite" value="sqlite" />
                    <el-option label="Clickhouse" value="clickhouse" />
                    <el-option label="SQLServer" value="sqlserver" />
                </el-select>
            </el-form-item>

            <template v-if="formData.dbType !== 'sqlite'">
                <el-form-item label="主机名" prop="host">
                    <el-input v-model="formData.host" />
                </el-form-item>

                <el-form-item label="端口号" prop="port">
                    <el-input-number v-model="formData.port" :min="1" :max="65535" />
                </el-form-item>

                <el-form-item label="用户名" prop="username">
                    <el-input v-model="formData.username" />
                </el-form-item>

                <el-form-item label="密码" prop="password">
                    <el-input v-model="formData.password" type="password" show-password />
                </el-form-item>
            </template>

            <el-form-item :label="formData.dbType === 'sqlite' ? '数据库路径' : '数据库名'" prop="database" required>
                <el-input v-model="formData.database" v-if="formData.dbType !== 'sqlite'" />
                <div v-else>
                    <el-input v-model="formData.database" readonly />
                    <el-button @click="selectDatabaseFile">选择文件</el-button>
                </div>
            </el-form-item>

            <el-form-item label="配置参数">
                <el-input v-model="formData.extraParams" placeholder="数据源的额外配置参数，格式为：param1=value1&param2=value1" />
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
import { open } from '@tauri-apps/plugin-dialog'
import type { FormInstance, FormRules } from 'element-plus'
import { dataSourceService, type DataSource } from '../services/DataSourceService'


const props = defineProps<{
    modelValue: boolean
    editData?: DataSource | null
}>()

const emit = defineEmits(['update:modelValue', 'success'])

const formRef = ref<FormInstance>()
const visible = ref(props.modelValue)
const formData = ref<Partial<DataSource>>({
    dbType: 'mysql',
    port: 3306
})

const rules = ref<FormRules>({
    name: [{ required: true, message: '请输入名称', trigger: 'blur' }],
    host: [
        { required: true, message: '请输入主机名', trigger: 'blur' },
    ],
    port: [
        { required: true, message: '请输入端口号', trigger: 'blur' },
        { type: 'number', min: 1, max: 65535, message: '端口号范围1-65535' }
    ],
    username: [{ required: true, message: '请输入用户名', trigger: 'blur' }],
    password: [{ required: true, message: '请输入密码', trigger: 'blur' }],
    database: [{ required: true, message: '请输入数据库名/路径', trigger: 'blur' }]
})

watch(() => props.modelValue, (val) => {
    visible.value = val
    if (val) {
        formData.value = props.editData
            ? { ...props.editData }
            : { dbType: 'mysql', port: 3306 }
    }
})

watch(visible, (val) => {
    emit('update:modelValue', val)
})

const handleClose = () => {
    formRef.value?.resetFields()
}

const selectDatabaseFile = async () => {
    try {
        const selected = await open({
            multiple: false,
            filters: [{
                name: 'SQLite Database',
                extensions: ['db', 'sqlite', 'sqlite3']
            }]
        })
        if (selected) {
            formData.value.database = selected
        }
    } catch (error) {
        console.error('选择文件失败:', error)
    }
}

const handleSubmit = async () => {
    if (!formRef.value) return
    const valid = await formRef.value.validate()
    if (!valid) return

    try {
        const data = formData.value as DataSource
        if (formData.value.id) {
            dataSourceService.update(data)
        } else {
            dataSourceService.create(data)
        }
        emit('success')
        visible.value = false
    } catch (error) {
        console.error('保存失败:', error)
    }
}
</script>

<style lang="scss" scoped>
:deep(.el-dialog) {
    background: #252526;

    .el-dialog__title {
        color: #d4d4d4;
    }

    .el-form-item__label {
        color: #888;
    }

    .el-input__inner {
        background: #333;
        border-color: #444;
        color: #d4d4d4;

        &:focus {
            border-color: #555;
        }
    }

    .el-input-number__decrease,
    .el-input-number__increase {
        background: #333;
        border-color: #444;
        color: #888;
    }
}
</style>