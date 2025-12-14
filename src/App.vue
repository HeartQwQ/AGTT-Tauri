<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
// 如果是在 Tauri 环境，推荐用这个 fetch，或者用原生 window.fetch 也可以
import { fetch } from '@tauri-apps/plugin-http';

const content = ref("等待数据..."); // 用于显示页面内容
const count = ref(0);              // 轮询计数器
let timer: number | null = null;   // 定时器 ID

// 核心：请求函数
const fetchData = async () => {
  try {
    // 请求你的后端状态接口
    const response = await fetch('http://127.0.0.1:14242/health', {
      method: 'GET',
    });

    const data = await response.json();

    count.value++;
    // 将获取到的 JSON 对象转为字符串显示
    content.value = JSON.stringify(data, null, 2);

  } catch (error) {
    content.value = `请求出错: ${error}`;
  }
};

// 生命周期：组件挂载时开始轮询
onMounted(() => {
  console.log("开始轮询...");
  // 立即执行一次
  fetchData();
  // 设置定时器，每 1000 毫秒 (1秒) 执行一次
  timer = window.setInterval(fetchData, 1000);
});

// 生命周期：组件卸载时清除定时器 (防止内存泄漏)
onUnmounted(() => {
  if (timer) {
    clearInterval(timer);
    timer = null;
    console.log("停止轮询");
  }
});
</script>

<template>
  <div style="padding: 20px; font-family: monospace;">
    <h1>最简轮询测试</h1>
    <p>请求次数: {{ count }}</p>

    <hr />

    <h3>后端返回的数据:</h3>
    <pre style="background: #f4f4f4; padding: 10px; border-radius: 5px;">{{ content }}</pre>
  </div>
</template>