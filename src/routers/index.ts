import { createMemoryHistory, createRouter } from 'vue-router'
import { SquareTerminal, House } from 'lucide-vue-next'

export const routes = [
  {
    path: '/',
    redirect: '/home'
  },
  {
    path: '/home',
    component: () => import('@/pages/Home.vue'),
    meta: {
      title: '首页',
      icon: House,
      order: 0,
    }
  },
  {
    path: '/reward',
    meta: {
      title: '奖励遍历',
      icon: SquareTerminal,
      order: 1,
    },
    children: [
      {
        path: 'redeem',
        component: () => import('@/pages/reward/Redeem.vue'),
        meta: {
          title: '兑换商店',
          order: 0
        }
      },
      {
        path: 'qt',
        component: () => import('@/pages/reward/Qt.vue'),
        meta: {
          title: '单抽/十连',
          order: 1
        }
      },
      {
        path: 'qt',
        component: () => import('@/pages/reward/Qt.vue'),
        meta: {
          title: '分解',
          order: 1
        }
      },
      {
        path: 'table',
        component: () => import('@/pages/reward/Table.vue'),
        meta: {
          title: '表格',
          order: 1
        }
      },
    ]
  }
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})