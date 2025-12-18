<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { InputGroup, InputGroupAddon, InputGroupButton, InputGroupInput } from "@/components/ui/input-group"

import { useForm } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/zod'
import * as z from 'zod'

import { Button } from '@/components/ui/button'
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'

// ========== 表单验证 ============
const formSchema = toTypedSchema(z.object({
  video_file: z.string().min(1, { message: "视频文件路径不能为空" }),
}))

const form = useForm({
  validationSchema: formSchema,
  initialValues: {
    video_file: '',
  }
})

const { handleSubmit, setFieldValue, meta } = form

const onSubmit = handleSubmit((values) => {
  console.log('表单已提交!', values)
})

// ========== 文件选择对话框 ============
import { open } from '@tauri-apps/plugin-dialog';
import { ref } from 'vue'

const video_fileName = ref('');

const handleSelectVideo = async () => {
  const selected = await open({
    multiple: false,
    directory: false,
    filters: [{
      name: '视频',
      extensions: ['mp4', 'mkv', 'avi', 'mov', 'wmv', 'flv', 'wmv', 'rmvb', '3gp', 'ogg', 'm4v', 'm4a', 'm4p', 'm4r', 'm4b', 'm4e', 'm4o', 'm4s', 'm4vh', 'm4vf', 'm4va', 'm4vz', 'm4vm', 'm4vw',]
    }]
  });

  if (selected) {
    const fullPath = Array.isArray(selected) ? selected[0] : selected;

    // 获取文件名
    const fileName = fullPath.split('\\').pop();
    video_fileName.value = fileName;

    // 设置表单字段值
    setFieldValue('video_file', fullPath);

    console.log('选择的文件完整路径:', fullPath);
  } else {

    // video_fileName.value = '';
    // setFieldValue('video_file', '');
    console.log('用户取消了文件选择');
  }
};

import { CircleX } from "lucide-vue-next"
</script>

<template>
  <div class="h-full w-full p-4 gap-3 flex">
    <div class="w-78 flex">
      <Card class="flex-1">
        <CardHeader>
          <CardTitle>奖励遍历-兑换商店</CardTitle>
          <CardDescription>配置好以下参数后再运行</CardDescription>
        </CardHeader>
        <CardContent class="flex-1">
          <form @submit="onSubmit" class="flex-col gap-4">

            <FormField v-slot="{ componentField }" name="video_file">
              <FormItem>
                <FormControl>
                  <div class="flex w-full max-w-sm items-center gap-1.5">
                    <InputGroup>
                      <InputGroupAddon>
                        <InputGroupText>选择视频：</InputGroupText>
                      </InputGroupAddon>
                      <InputGroupInput type="text" placeholder="视频文件" v-bind="componentField" :value="video_fileName"
                        read-only @click="handleSelectVideo" />
                      <InputGroupAddon align="inline-end">
                        <InputGroupButton aria-label="Copy" title="清空" size="icon-xs"
                          @click="video_fileName = ''; setFieldValue('video_file', '');">
                          <CircleX />
                        </InputGroupButton>
                      </InputGroupAddon>
                    </InputGroup>
                    <!-- <Button @click="handleSelectVideo">
                      选择视频
                    </Button> -->
                  </div>
                </FormControl>
                <FormDescription>
                </FormDescription>
                <FormMessage />
              </FormItem>
            </FormField>

            <Button type="submit">
              提交
            </Button>
          </form>
        </CardContent>
        <CardFooter>
          卡片底部
        </CardFooter>
      </Card>
    </div>
    <div class="flex-1 bg-blue-500">
    </div>
  </div>
</template>