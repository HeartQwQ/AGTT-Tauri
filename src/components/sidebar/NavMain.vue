<script setup lang="ts">
import type { LucideIcon } from "lucide-vue-next"
import { ChevronRight } from "lucide-vue-next"
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible"
import {
  SidebarGroup,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
} from "@/components/ui/sidebar"

defineProps<{
  // items: {
  //   title: string
  //   url: string
  //   icon?: LucideIcon
  //   isActive?: boolean
  //   items?: {
  //     title: string
  //     url: string
  //   }[]
  // }[]
  items: any[]
}>()
</script>

<template>
  <SidebarGroup>
    <SidebarMenu>
      <Collapsible
        v-for="item in items"
        :key="item"
        as-child
        :default-open="true"
        class="group/collapsible"
      >
        <SidebarMenuItem>
          <SidebarMenuSubButton as-child>
            <RouterLink to="#">a</RouterLink>
          </SidebarMenuSubButton>
        </SidebarMenuItem>
      </Collapsible>
      <!-- <Collapsible
        v-for="item in items"
        :key="item"
        as-child
        :default-open="false"
        class="group/collapsible"
      >
        <SidebarMenuItem>
          <CollapsibleTrigger as-child>
            <SidebarMenuButton :tooltip="item.meta.title">
              <component :is="item.icon" v-if="item.icon" />
              <span>{{ item.title }}</span>
              <ChevronRight class="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
            </SidebarMenuButton>
          </CollapsibleTrigger>
          <CollapsibleContent>
            <SidebarMenuSub>
              <SidebarMenuSubItem v-for="subItem in item.items" :key="subItem.title">
                <SidebarMenuSubButton as-child>
                  <RouterLink :to="subItem.url">{{ subItem.title }}</RouterLink>
                </SidebarMenuSubButton>
              </SidebarMenuSubItem>
            </SidebarMenuSub>
          </CollapsibleContent>
        </SidebarMenuItem>
      </Collapsible> -->
    </SidebarMenu>
  </SidebarGroup>
</template>
