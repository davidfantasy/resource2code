<template>
    <div class="resource-explorer">

        <el-input v-model="searchQuery" placeholder="搜索文件..." clearable class="search-input">
            <template #prefix>
                <el-icon>
                    <Search />
                </el-icon>
            </template>
        </el-input>

        <el-tree ref="treeRef" :data="treeData" :props="defaultProps" node-key="id" :expand-on-click-node="false"
            :filter-node-method="filterNode" @node-click="handleTreeNodeClick" class="resource-tree">
            <template #default="{ node, data }">
                <span class="tree-node">
                    <div class="node-content">
                        <!-- 根据类型显示不同图标 -->
                        <el-icon class="type-icon">
                            <Folder v-if="data.isFolder" />
                            <Document v-if="!data.isFolder" />
                        </el-icon>
                        <span class="label">{{ node.label }}</span>
                    </div>
                    <el-button v-if="data.type == 'database-root'" @click.stop="handleDatabaseCreate"
                        class="refresh-btn" :icon="Plus" type="text" title="新增数据源" />
                    <el-button v-if="data.type == 'database'" @click.stop="handleDatabaseRemove(data.id)"
                        class="refresh-btn" :icon="Delete" type="text" title="删除数据源" />
                    <el-button v-if="data.type == 'database'" class="refresh-btn"
                        @click.stop="handleDatabaseEdit(data.id)" :icon="Edit" type="text" title="修改" />
                    <!-- 根节点刷新按钮 -->
                    <el-button v-if="data.id == 'source-root' || data.id == 'database-root'"
                        @click.stop="handleRefresh(data.type)" class="refresh-btn" :icon="Refresh" type="text"
                        title="刷新目录" />
                    <el-button v-if="data.id == 'source-root'" @click.stop="handleOpenFolder" class="refresh-btn"
                        :icon="FolderOpened" type="text" title="打开新项目" />
                </span>
            </template>
        </el-tree>
    </div>
    <DataSourceForm v-model="showDatasourceForm" :edit-data="currentEditDsData" @success="reloadDatabaseConnections" />
</template>


<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { Folder, Document, Search, Refresh, Plus, Delete, Edit, FolderOpened } from '@element-plus/icons-vue'
import DataSourceForm from '@/components/DataSourceForm.vue'
import { dataSourceService, type DataSource } from '../services/DataSourceService'
import { ElMessage, ElMessageBox } from 'element-plus';
import { ResourceMeta } from '../services/dto';
import { open } from '@tauri-apps/plugin-dialog'


// 定义树节点的类型
interface TreeNode {
    id: string;
    parentId: string;
    label: string;
    type?: string;
    isFolder?: boolean;
    children?: TreeNode[];
}

const emit = defineEmits(['resource-add'])

// 拆分数据源并添加类型声明
const sourceData = ref<TreeNode[]>([])
const databaseData = ref<TreeNode[]>([])
const treeRef = ref()
const searchQuery = ref('')
const showDatasourceForm = ref(false)
const currentEditDsData = ref<DataSource | null>(null)
const rootSourcePath = ref('');

const defaultProps = {
    children: 'children',
    label: 'label'
}

// 组合树形数据
const treeData = computed(() => [
    {
        id: 'database-root',
        label: '数据库连接',
        type: 'database-root',
        isFolder: true,
        children: databaseData.value
    },
    {
        id: 'source-root',
        label: '源代码',
        type: 'source-root',
        isFolder: true,
        children: sourceData.value
    },
])

// 加载数据库连接模拟数据
const reloadDatabaseConnections = async () => {
    let dbs = await dataSourceService.list();
    try {
        databaseData.value = [];
        dbs.forEach(async db => {
            // 修改 dbNode 定义部分
            let dbNode: TreeNode = {
                id: db.id.toString(),
                parentId: 'database-root',
                label: db.name,
                type: 'database',
                isFolder: true,
                children: [] // 显式初始化为 []
            };
            try {
                const tables = await invoke<string[]>('get_tables', { ds: db });
                dbNode.children = tables.map(table => ({
                    id: `${table}`,
                    parentId: db.id.toString(),
                    label: `${table}`,
                    type: 'table',
                    isFolder: false
                })) as TreeNode[];
            } catch (error) {
                ElMessage.error('获取数据表失败:' + error)
            }
            databaseData.value.push(dbNode);
        });
    } catch (error) {
        ElMessage.error('加载数据库失败:' + error)
    }
}

// 刷新功能
const handleRefresh = async (type: string) => {
    if (type === 'source-root') {
        await loadFileSystem(rootSourcePath.value)
    } else {
        await reloadDatabaseConnections()
    }
    ElMessage.success('数据已刷新')
}

// 修改后的加载文件系统方法
const loadFileSystem = async (path: string) => {
    try {
        if (!path) {
            return;
        }
        const data = await invoke('get_file_system', { path: path })
        sourceData.value = data as TreeNode[]
    } catch (error) {
        ElMessage.error('加载文件系统失败:' + error)
    }
}

watch(searchQuery, (val) => {
    treeRef.value!.filter(val)
})

// 节点过滤方法
const filterNode = (value: string, data: TreeNode): boolean => {
    if (!value) return true
    return data.label.toLowerCase().includes(value.toLowerCase()) && !data.isFolder
}

const handleDatabaseRemove = async (id: string) => {
    await dataSourceService.delete(id);
    reloadDatabaseConnections();
    ElMessage.success('数据源删除成功');
}

const handleDatabaseCreate = () => {
    currentEditDsData.value = null;
    showDatasourceForm.value = true;
}
const handleDatabaseEdit = async (id: string) => {
    const ds = await dataSourceService.find(id);
    if (ds) {
        currentEditDsData.value = ds;
        showDatasourceForm.value = true;
    }
}

const handleTreeNodeClick = (node: TreeNode) => {
    if (!node.isFolder) {
        ElMessageBox.confirm('确定添加该资源作为附加内容吗?', '提示', {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'info'
        }).then(() => {
            if (node.type == 'table') {
                let resource: ResourceMeta = { name: node.label, resourceType: 'table', data: node.parentId }
                emit('resource-add', resource)
            } else {
                let resource: ResourceMeta = { name: node.label, resourceType: 'file', data: node.id }
                emit('resource-add', resource)
            }
        }).catch(() => {
        });
    }
}

const handleOpenFolder = async () => {
    const selected = await open({
        directory: true,
        multiple: false,
        title: '选择项目目录'
    })
    if (selected) {
        rootSourcePath.value = selected;
        loadFileSystem(selected)
        await invoke('set_config', { key: "root_source_path", value: selected })
    }
}

onMounted(async () => {
    rootSourcePath.value = await invoke('get_config', { key: "root_source_path" })
    loadFileSystem(rootSourcePath.value)
    reloadDatabaseConnections()
})
</script>



<style lang="scss" scoped>
.resource-explorer {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--el-bg-color);
    color: var(--el-text-color-primary);
}

.search-input {
    margin: 12px;
    width: calc(100% - 24px);

    :deep(.el-input__inner) {
        background-color: var(--el-bg-color);
        color: var(--el-text-color-primary);
        border-color: var(--el-border-color);

        &:focus {
            border-color: var(--el-color-primary);
        }
    }

    .el-icon {
        color: var(--el-text-color-secondary);
    }
}

.resource-tree {
    flex: 1;
    padding: 0 12px 12px;
    overflow: auto;
    background: var(--el-bg-color);

    :deep(.el-tree-node__content) {
        height: 36px;
        transition: all 0.2s;

        &:hover {
            background-color: var(--el-fill-color-light);
        }
    }

    .tree-node {
        display: flex;
        justify-content: space-between;
        align-items: center;
        width: 100%;

        .node-content {
            display: flex;
            align-items: center;
            flex: 1;

            .type-icon {
                margin-right: 8px;
                color: var(--el-text-color-secondary);
            }

            .label {
                color: var(--el-text-color-primary);
            }
        }

        .refresh-btn {
            padding: 0;
            color: var(--el-text-color-secondary);
            visibility: hidden;

            &:hover {
                color: var(--el-color-primary);
            }
        }

        &:hover .refresh-btn {
            visibility: visible;
        }
    }
}
</style>