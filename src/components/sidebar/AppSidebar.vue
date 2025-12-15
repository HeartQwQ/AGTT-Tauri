<script setup lang="ts">
import type { SidebarProps } from "@/components/ui/sidebar"

import {
  AudioWaveform,
  Command,
  GalleryVerticalEnd,
} from "lucide-vue-next"
import NavMain from "@/components/sidebar/NavMain.vue"
import NavUser from "@/components/sidebar/NavUser.vue"
import TeamSwitcher from "@/components/sidebar/TeamSwitcher.vue"

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
} from "@/components/ui/sidebar"

const props = withDefaults(defineProps<SidebarProps>(), {
  collapsible: "icon",
})
import { routes } from "@/routers/index"

// 用户和团队数据
const data = {
  user: {
    name: "用户名",
    email: "邮箱@tencent.com",
    avatar: "/avatars/shadcn.jpg",
  },
  teams: [
    {
      name: "CJ GAME",
      logo: GalleryVerticalEnd,
      plan: "Game For Peace",
    },
    {
      name: "和平精英_潘多拉",
      logo: AudioWaveform,
      plan: "Game For Peace",
    },
    {
      name: "和平UE5升级",
      logo: Command,
      plan: "Game For Peace",
    },
  ],
}
</script>

<template>
  <Sidebar v-bind="props">
    <SidebarHeader>
      <TeamSwitcher :teams="data.teams" />
    </SidebarHeader>
    <SidebarContent>
      <NavMain :items="routes" />
      <!-- <NavProjects :projects="routes" /> -->
    </SidebarContent>
    <SidebarFooter>
      <NavUser :user="data.user" />
    </SidebarFooter>
    <SidebarRail />
  </Sidebar>
</template>
