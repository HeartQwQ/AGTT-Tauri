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

const props = defineProps<{
  items: any[]
}>()
import { computed } from 'vue';
const menuItems = computed(() => {
  return props.items.filter(item =>
    item.path !== '/' && item.meta && item.meta.title
  );
});
</script>

<template>
  <SidebarGroup>
    <SidebarMenu>
      <template v-for="item in menuItems" :key="item.path">

        <Collapsible v-if="item.children && item.children.length > 0" as-child :default-open="false"
          class="group/collapsible">
          <SidebarMenuItem>
            <CollapsibleTrigger as-child>
              <SidebarMenuButton :tooltip="item.meta.title">
                <component :is="item.meta.icon" v-if="item.meta.icon" class="w-5 h-5" />
                <span>{{ item.meta.title }}</span>
                <ChevronRight
                  class="ml-auto w-4 h-4 transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
              </SidebarMenuButton>
            </CollapsibleTrigger>

            <CollapsibleContent>
              <SidebarMenuSub>
                <SidebarMenuSubItem v-for="subItem in item.children" :key="subItem.path">
                  <SidebarMenuSubButton as-child>
                    <RouterLink :to="`${item.path}/${subItem.path}`">
                      {{ subItem.meta.title }} </RouterLink>
                  </SidebarMenuSubButton>
                </SidebarMenuSubItem>
              </SidebarMenuSub>
            </CollapsibleContent>
          </SidebarMenuItem>
        </Collapsible>

        <SidebarMenuItem v-else>
          <SidebarMenuButton :tooltip="item.meta.title" as-child>
            <RouterLink :to="item.path">
              <component :is="item.meta.icon" v-if="item.meta.icon" class="w-5 h-5" />
              <span>{{ item.meta.title }}</span>
            </RouterLink>
          </SidebarMenuButton>
        </SidebarMenuItem>

      </template>
    </SidebarMenu>
  </SidebarGroup>
</template>
