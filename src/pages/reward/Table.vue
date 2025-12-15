<script setup lang="ts">
import { onMounted, shallowRef } from 'vue' // 1. 引入 onMounted
import { UniverSheetsCorePreset } from '@univerjs/preset-sheets-core'
import UniverPresetSheetsCoreZhCN from '@univerjs/preset-sheets-core/locales/zh-CN'
import { createUniver, LocaleType, mergeLocales } from '@univerjs/presets'

import '@univerjs/preset-sheets-core/lib/index.css'

// 步骤 1.1: 创建一个 shallowRef 来存储 univerAPI 实例
const univerApiRef = shallowRef<any>(null)

// 2. 将 createUniver/createWorkbook 逻辑移动到 onMounted 内部
onMounted(() => {
    const { univer, univerAPI } = createUniver({
        locale: LocaleType.ZH_CN,
        locales: {
            [LocaleType.ZH_CN]: mergeLocales(
                UniverPresetSheetsCoreZhCN,
            ),
        },
        presets: [
            UniverSheetsCorePreset({
                container: 'univer-container',
            }),
        ],
    })

    univerApiRef.value = univerAPI
    // 步骤 3：定义工作簿配置，包含多个工作表
    const workbookConfig = {
        // **核心配置：定义工作表列表**
        sheets: {
            // 第一个工作表配置
            'sheet-001': {
                id: 'sheet-001',
                name: '兑换商店', // 预期的名称
                cellData: {}, // 可选：初始化单元格数据
                dataGrid: {}, // 可选：初始化数据网格设置
                // 其他工作表属性...
            },
            // 第二个工作表配置
            'sheet-002': {
                id: 'sheet-002',
                name: '单抽', // 预期的名称
                cellData: {},
                dataGrid: {},
                // 其他工作表属性...
            },
            // 第三个工作表配置
            'sheet-003': {
                id: 'sheet-003',
                name: '十连',
                cellData: {},
                dataGrid: {},
            },
        },

        // 可选：指定初始化时激活哪个工作表
        activeSheetId: 'sheet-002',
    }

    // 调用 createWorkbook，传入配置对象
    univerAPI.createWorkbook(workbookConfig)
})
</script>

<template>
    <div id="univer-container" class="w-full h-full"></div>
</template>