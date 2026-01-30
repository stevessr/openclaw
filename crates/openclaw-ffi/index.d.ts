export interface JsMessage {
  id: string;
  session_id: string;
  content: string;
  timestamp: string;
}

export interface JsConfig {
  gateway_host: string;
  gateway_port: number;
  gateway_mode: string;
}

/**
 * Initialize the FFI bridge
 * @returns Initialization message
 */
export function init(): string;

/**
 * Get version information
 * @returns Version string
 */
export function getVersion(): string;

/**
 * Create a new message ID
 * @returns UUID string
 */
export function createMessageId(): string;

/**
 * Create a new session ID
 * @returns UUID string
 */
export function createSessionId(): string;

/**
 * Create a new message
 * @param sessionId Session ID
 * @param content Message content
 * @returns Created message
 */
export function createMessage(sessionId: string, content: string): JsMessage;

/**
 * Load configuration
 * @returns Configuration object
 */
export function loadConfig(): JsConfig;
