<template>
    <el-dialog v-model="visible" :title="formData.id ? '编辑数据源' : '新建数据源'" width="600px" @close="handleClose">
        <el-form ref="formRef" :model="formData" :rules="rules" label-width="100px" label-position="left">
            <el-form-item label="名称" prop="name">
                <el-input v-model="formData.name" />
            </el-form-item>

            <el-form-item label="数据库类型" prop="dbType">
                <el-select v-model="formData.dbType" disabled>
                    <el-option label="MySQL" value="mysql" />
                </el-select>
            </el-form-item>

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

            <el-form-item label="数据库名">
                <el-input v-model="formData.database" />
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
    password: [{ required: true, message: '请输入密码', trigger: 'blur' }]
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