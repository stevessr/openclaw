// Example: Using OpenClaw Rust FFI from TypeScript
// 示例：从 TypeScript 使用 OpenClaw Rust FFI

// Import the FFI bindings
// 导入 FFI 绑定
import { 
  init, 
  getVersion, 
  createMessageId, 
  createSessionId, 
  createMessage, 
  loadConfig 
} from '@openclaw/ffi';

// Initialize the Rust FFI
// 初始化 Rust FFI
console.log(init());

// Get version information
// 获取版本信息
console.log('Version:', getVersion());

// Create IDs
// 创建 ID
const messageId = createMessageId();
const sessionId = createSessionId();
console.log('Message ID:', messageId);
console.log('Session ID:', sessionId);

// Create a message
// 创建消息
const message = createMessage(sessionId, 'Hello from TypeScript!');
console.log('Created message:', message);

// Load configuration
// 加载配置
const config = loadConfig();
console.log('Config:', config);

// Use in a TypeScript plugin
// 在 TypeScript 插件中使用
export async function handleMessage(content: string) {
  const sessionId = createSessionId();
  const message = createMessage(sessionId, content);
  
  console.log(`Processing message ${message.id} in session ${message.session_id}`);
  
  // Your plugin logic here
  // 您的插件逻辑在这里
  
  return message;
}

// Example plugin integration
// 示例插件集成
export const myPlugin = {
  name: 'my-rust-plugin',
  version: '1.0.0',
  
  async onMessage(content: string) {
    const message = await handleMessage(content);
    return `Processed: ${message.content}`;
  },
  
  getInfo() {
    return {
      rustVersion: getVersion(),
      gatewayConfig: loadConfig()
    };
  }
};
