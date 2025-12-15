<script lang="ts">
export const description
  = "A sidebar that collapses to icons."
export const iframeHeight = "800px"
export const containerClass = "w-full h-full"
</script>

<script setup lang="ts">
import AppSidebar from "@/components/sidebar/AppSidebar.vue"
import ChevronDownIcon from '~icons/radix-icons/chevron-down'
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Separator } from "@/components/ui/separator"
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar"


import { computed } from 'vue';
import { useRoute, useRouter } from 'vue-router'
import { routes } from "@/routers/index"

const route = useRoute(); // 获取当前路由信息
const router = useRouter();

const breadcrumbList = computed(() => {
  const matchedRoutes = route.matched;

  const crumbs = matchedRoutes
    .filter(record => record.meta && record.meta.title)
    .map(record => {
      // ... (isParent, children, to 逻辑保持不变)
      const hasChildren = record.children && record.children.length > 0;
      return {
        path: record.path,
        title: record.meta.title,
        isParent: hasChildren,
        to: !hasChildren && record.path !== route.path ? record.path : '',
        children: hasChildren ? record.children.filter(c => c.meta?.title) : []
      };
    });

  // 核心优化：避免重复插入“首页”
  // 1. 检查当前面包屑列表中是否已包含路径为 '/home' 的项
  const homeExists = crumbs.find(item => item.path === '/home');

  // 2. 仅当列表不包含 '/home' 时，才执行 unshift 插入固定首页
  if (!homeExists) {
    crumbs.unshift({ // 插入固定的“首页”
      path: '/home',
      title: '首页',
      to: '/home',
      isParent: false,
      children: []
    });
  }

  // 3. 特殊处理：如果首页存在，但不是第一项（通常不会发生，但作为严谨性检查）
  // 确保 /home 始终是第一项 (如果它存在的话)

  return crumbs;
});

console.log(breadcrumbList)
</script>

<template>
  <SidebarProvider>
    <AppSidebar />
    <SidebarInset>
      <header
        class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12">
        <div class="flex items-center gap-2 px-4">
          <SidebarTrigger class="-ml-1" />
          <Separator orientation="vertical" class="mr-2 h-4" />
          <Breadcrumb>
            <BreadcrumbList>
              <template v-for="(item, index) in breadcrumbList" :key="item.path">
                <BreadcrumbItem>
                  <template v-if="item.isParent">
                    <DropdownMenu>
                      <DropdownMenuTrigger class="flex items-center gap-1 cursor-pointer">
                        {{ item.title }}
                        <ChevronDownIcon />
                      </DropdownMenuTrigger>
                      <DropdownMenuContent align="start">
                        <DropdownMenuItem v-for="child in item.children" :key="child.path"
                          @click="router.push(item.path + '/' + child.path)" class="cursor-pointer">
                          {{ child.meta.title }}
                        </DropdownMenuItem>
                      </DropdownMenuContent>
                    </DropdownMenu>
                  </template>

                  <template v-else-if="item.to">
                    <BreadcrumbLink :href="item.to">{{ item.title }}</BreadcrumbLink>
                  </template>
                  <template v-else>
                    <BreadcrumbPage>{{ item.title }}</BreadcrumbPage>
                  </template>
                </BreadcrumbItem>

                <BreadcrumbSeparator v-if="index < breadcrumbList.length - 1" />
              </template>
            </BreadcrumbList>
          </Breadcrumb>
        </div>
      </header>
      <div class="flex flex-1 h-[calc(100vh-64px)]">
        <RouterView />
      </div>
    </SidebarInset>
  </SidebarProvider>
</template>
