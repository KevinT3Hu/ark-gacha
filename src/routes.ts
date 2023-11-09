import { Router, createRouter, createWebHashHistory } from 'vue-router';

const router:Router = createRouter({
    history: createWebHashHistory(),
    routes:[
        { path: '/', name: 'login', component: () => import('./routes/Login.vue') },
        { path: '/main', name: 'main', component: () => import('./routes/Main.vue') },
    ]
})
export default router;
