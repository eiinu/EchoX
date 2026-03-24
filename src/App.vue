<script setup lang="ts">
import { ref, onMounted } from 'vue';

interface TerminalEntry {
  id: number;
  command: string;
  output: string;
  timestamp: string;
}

const commandInput = ref('');
const terminalEntries = ref<TerminalEntry[]>([]);
let entryId = 0;

const executeCommand = () => {
  const command = commandInput.value.trim();
  if (!command) return;

  // 添加命令到终端
  const newEntry: TerminalEntry = {
    id: entryId++,
    command,
    output: `执行命令: ${command}\n这是模拟的命令输出`,
    timestamp: new Date().toLocaleTimeString(),
  };
  terminalEntries.value.push(newEntry);

  // 清空输入
  commandInput.value = '';

  // 滚动到底部
  setTimeout(() => {
    const terminal = document.querySelector('.terminal-output');
    if (terminal) {
      terminal.scrollTop = terminal.scrollHeight;
    }
  }, 0);
};

const handleKeyPress = (event: KeyboardEvent) => {
  if (event.key === 'Enter') {
    executeCommand();
  }
};

onMounted(() => {
  // 添加欢迎信息
  terminalEntries.value.push({
    id: entryId++,
    command: '',
    output: 'Welcome to EchoX Terminal\nType a command and press Enter to execute',
    timestamp: new Date().toLocaleTimeString(),
  });
});
</script>

<template>
  <div class="app-frame">
    <main class="shell">
      <header class="topbar">
        <h1>EchoX</h1>
        <div class="toolbar-group">
          <div class="status">Terminal</div>
        </div>
      </header>

      <section class="terminal">
        <div class="terminal-output">
          <div v-for="entry in terminalEntries" :key="entry.id" class="terminal-entry">
            <div v-if="entry.command" class="command-line">
              <span class="prompt">$</span>
              <span class="command">{{ entry.command }}</span>
            </div>
            <div class="output">{{ entry.output }}</div>
          </div>
        </div>
        <div class="terminal-input">
          <span class="prompt">$</span>
          <input
            v-model="commandInput"
            type="text"
            @keypress="handleKeyPress"
            placeholder="Type a command..."
            autofocus
          />
        </div>
      </section>
    </main>
  </div>
</template>

<style scoped>
:global(*) { box-sizing: border-box; }
:global(html),
:global(body),
:global(#app) {
  margin: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}
:global(body) {
  font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
  background: #1e1e1e;
  color: #d4d4d4;
}
:global(button), :global(input), :global(textarea), :global(select) { font: inherit; }

.app-frame {
  width: 100%;
  height: 100%;
  overflow: hidden;
  display: flex;
  justify-content: stretch;
}
.shell {
  flex: 1;
  width: 100%;
  min-width: 0;
  min-height: 0;
  height: 100%;
  padding: 14px;
  display: grid;
  grid-template-rows: auto 1fr;
  gap: 14px;
  overflow: hidden;
}
.topbar {
  background: #252526;
  border: 1px solid #3e3e42;
  border-radius: 12px;
  padding: 14px 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  flex-wrap: wrap;
}
.topbar h1 {
  margin: 0;
  font-size: 22px;
  font-weight: 700;
  color: #ffffff;
}
.toolbar-group {
  display: flex;
  align-items: center;
  gap: 12px;
}
.status {
  font-size: 13px;
  font-weight: 600;
  color: #969696;
}
.terminal {
  background: #1e1e1e;
  border: 1px solid #3e3e42;
  border-radius: 12px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.terminal-output {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  font-size: 14px;
  line-height: 1.5;
}
.terminal-entry {
  margin-bottom: 12px;
}
.command-line {
  display: flex;
  align-items: center;
  margin-bottom: 4px;
}
.prompt {
  color: #00ff00;
  margin-right: 8px;
  font-weight: bold;
}
.command {
  color: #ffffff;
}
.output {
  color: #d4d4d4;
  margin-left: 16px;
}
.terminal-input {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-top: 1px solid #3e3e42;
  background: #252526;
}
.terminal-input input {
  flex: 1;
  border: none;
  background: transparent;
  color: #ffffff;
  outline: none;
  margin-left: 8px;
}
.terminal-input input::placeholder {
  color: #666666;
}
</style>
