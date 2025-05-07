import { createRouter, createWebHistory } from 'vue-router';
import AIChat from '@/components/AIChat.vue';

const routes = [
    {
        path: '/', // 默认路由路径
        name: 'AIChat',
        component: AIChat, // 指向AIChat组件
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;