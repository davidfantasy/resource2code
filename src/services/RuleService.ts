import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";

export interface Rule {
    id: string;
    name: string;
    content: string;
}

export const ruleService = {
    async list(): Promise<Rule[]> {
        try {
            const codeStandards = await invoke<Rule[]>('get_all_samples');
            return codeStandards;
        } catch (e) {
            ElMessage.error('查询规则失败:' + JSON.stringify(e));
            throw e;
        }
    },

    async create(data: Omit<Rule, 'id'>): Promise<Rule> {
        try {
            const newId = await invoke<string>('create_sample', {
                cs: {
                    ...data,
                    id: ""
                }
            });
            return { ...data, id: newId };
        } catch (e) {
            ElMessage.error('创建规则发生错误:' + JSON.stringify(e));
            throw e;
        }
    },

    async find(id: string): Promise<Rule | null> {
        try {
            const item = await invoke<Rule>('get_sample_by_id', { id });
            return item;
        } catch (e) {
            ElMessage.error('查询规则发生错误:' + JSON.stringify(e));
            return null;
        }
    },

    async update(data: Rule) {
        try {
            await invoke<boolean>('update_sample', {
                cs: data
            });
        } catch (e) {
            ElMessage.error('更新规则失败:' + JSON.stringify(e));
            throw e;
        }
    },

    async delete(id: String): Promise<boolean> {
        try {
            return await invoke<boolean>('delete_sample', { id });
        } catch (e) {
            ElMessage.error('删除规则发生错误:' + JSON.stringify(e));
            return false;
        }
    }
};