import { createMemoryHistory, createRouter } from 'vue-router'
import { SquareTerminal } from 'lucide-vue-next'

import Home from '../pages/Home.vue'
import RewardTraversal from '../pages/autoTest/RewardTraversal.vue'
import Qt from '../pages/autoTest/Qt.vue'

export const routes = [
  {
    path: '/',
    redirect: '/home'
  },
  {
    path: '/home',
    component: Home,
    meta: {
      title: '首页',
      icon: SquareTerminal,
      order: 0,
    }
  },
  {
    path: '/autoTest',
    meta: {
      title: '自动化',
      icon: SquareTerminal,
      order: 1,
    },
    children: [
      {
        path: 'reward-traversal',
        component: RewardTraversal,
        meta: {
          title: '奖励遍历',
          order: 0
        }
      },
      {
        path: 'qt',
        component: Qt,
        meta: {
          title: '其他',
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