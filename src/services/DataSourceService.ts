import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";

export interface DataSource {
    id: string;
    name: string;
    dbType: 'mysql';
    host: string;
    port: number;
    username: string;
    password: string;
    database?: string;
}

export const dataSourceService = {
    async list(): Promise<DataSource[]> {
        try {
            const datasourcs = await invoke<DataSource[]>('get_all_ds');
            return datasourcs;
        } catch (e) {
            ElMessage.error('查询数据源失败:' + e);
            throw new Error('查询数据源失败');
        }

    },

    async create(data: Omit<DataSource, 'id'>): Promise<DataSource> {
        try {
            const newId = await invoke<number>('create_ds', {
                ds: {
                    ...data,
                    id: ""
                }
            });
            return { ...data, id: newId.toString() };
        } catch (e) {
            ElMessage.error('创建数据源发生错误:' + e);
            throw new Error('创建数据源失败');
        }
    },

    async find(id: string): Promise<DataSource | null> {
        try {
            const item = await invoke<DataSource>('get_ds_by_id', { id });
            return item;
        } catch (e) {
            ElMessage.error('查询数据源发生错误:' + e);
            return null;
        }
    },

    async update(data: DataSource): Promise<DataSource | null> {
        const success = await invoke<boolean>('update_ds', {
            ds: data
        });
        return success ? data : null;
    },

    async delete(id: string): Promise<boolean> {
        try {
            return await invoke<boolean>('delete_ds', { id });
        } catch (e) {
            ElMessage.error('删除数据源发生错误:' + e);
            return false;
        }
    }
};
