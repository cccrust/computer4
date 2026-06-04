import '@testing-library/jest-dom'

// Mock @tauri-apps/api/core
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

// Mock @tauri-apps/plugin-fs
vi.mock('@tauri-apps/plugin-fs', () => ({
  readTextFile: vi.fn(),
  writeTextFile: vi.fn(),
  readDir: vi.fn(),
  mkdir: vi.fn(),
  remove: vi.fn(),
  rename: vi.fn(),
  exists: vi.fn(),
}))

// Mock @tauri-apps/api/path
vi.mock('@tauri-apps/api/path', () => ({
  documentDir: vi.fn().mockResolvedValue('/Users/test/Documents'),
  join: vi.fn().mockImplementation((a, b) => `${a}/${b}`),
}))

// Mock lucide-react
vi.mock('lucide-react', () => ({
  FolderOpen: () => 'FolderOpen',
  Save: () => 'Save',
  File: () => 'File',
  AlertCircle: () => 'AlertCircle',
}))