import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { renderHook } from '@testing-library/react'
import Editor from '../apps/editor'

// Import mocks
import * as fs from '@tauri-apps/plugin-fs'
import * as path from '@tauri-apps/api/path'
import * as core from '@tauri-apps/api/core'

const mockFs = vi.mocked(fs)
const mockPath = vi.mocked(path)
const mockCore = vi.mocked(core)

describe('Editor App', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockPath.documentDir.mockResolvedValue('/Users/test/Documents')
    mockPath.join.mockImplementation((a: string, b: string) => `${a}/${b}`)
  })

  describe('UI Rendering', () => {
    it('renders Editor with toolbar and empty content', () => {
      render(<Editor />)

      expect(screen.getByText('Open')).toBeInTheDocument()
      expect(screen.getByText('Save')).toBeInTheDocument()
      expect(screen.getByText('untitled.txt')).toBeInTheDocument()
      expect(screen.getByPlaceholderText('Open a file or start typing...')).toBeInTheDocument()
    })

    it('shows modified indicator when content changes', async () => {
      const { container } = render(<Editor />)
      const textarea = container.querySelector('textarea')
      if (textarea) {
        fireEvent.change(textarea, { target: { value: 'Hello World' } })
      }

      await waitFor(() => {
        expect(screen.getByText('untitled.txt •')).toBeInTheDocument()
      })
    })
  })

  describe('File Open Dialog', () => {
    it('opens file browser when Open button is clicked', async () => {
      mockFs.readDir.mockResolvedValue([])

      render(<Editor />)

      const openBtn = screen.getByText('Open')
      fireEvent.click(openBtn)

      await waitFor(() => {
        expect(screen.getByText('Open File')).toBeInTheDocument()
      })
    })

    it('loads directory contents when dialog opens', async () => {
      const mockEntries = [
        { name: 'folder1', isDirectory: true },
        { name: 'file1.txt', isDirectory: false },
        { name: 'file2.md', isDirectory: false },
      ]
      mockFs.readDir.mockResolvedValue(mockEntries as any)

      render(<Editor />)

      const openBtn = screen.getByText('Open')
      fireEvent.click(openBtn)

      await waitFor(() => {
        expect(mockFs.readDir).toHaveBeenCalledWith('/Users/test/Documents')
        expect(screen.getByText('folder1')).toBeInTheDocument()
        expect(screen.getByText('file1.txt')).toBeInTheDocument()
        expect(screen.getByText('file2.md')).toBeInTheDocument()
      })
    })

    it('navigates into directory when clicked', async () => {
      const parentEntries = [{ name: 'subdir', isDirectory: true }]
      const subdirEntries = [{ name: 'nested.txt', isDirectory: false }]

      mockFs.readDir
        .mockResolvedValueOnce(parentEntries as any)
        .mockResolvedValueOnce(subdirEntries as any)

      render(<Editor />)

      const openBtn = screen.getByText('Open')
      fireEvent.click(openBtn)

      await waitFor(() => {
        expect(screen.getByText('subdir')).toBeInTheDocument()
      })

      const subdirBtn = screen.getByText('subdir')
      fireEvent.click(subdirBtn)

      await waitFor(() => {
        expect(mockFs.readDir).toHaveBeenCalledTimes(2)
        expect(screen.getByText('nested.txt')).toBeInTheDocument()
      })
    })

    it('loads file content when file is clicked', async () => {
      const parentEntries = [{ name: 'test.txt', isDirectory: false }]
      mockFs.readDir.mockResolvedValue(parentEntries as any)
      mockFs.readTextFile.mockResolvedValue('Hello World Content')

      render(<Editor />)

      const openBtn = screen.getByText('Open')
      fireEvent.click(openBtn)

      await waitFor(() => {
        expect(screen.getByText('test.txt')).toBeInTheDocument()
      })

      const fileBtn = screen.getByText('test.txt')
      fireEvent.click(fileBtn)

      await waitFor(() => {
        expect(mockFs.readTextFile).toHaveBeenCalled()
        const textarea = screen.getByPlaceholderText('Open a file or start typing...') as HTMLTextAreaElement
        expect(textarea.value).toBe('Hello World Content')
        expect(screen.getByText('test.txt')).toBeInTheDocument()
      })
    })
  })

  describe('Save Functionality', () => {
    it('saves file with content', async () => {
      mockFs.writeTextFile.mockResolvedValue()

      render(<Editor />)

      const textarea = screen.getByPlaceholderText('Open a file or start typing...') as HTMLTextAreaElement
      fireEvent.change(textarea, { target: { value: 'New content' } })

      const saveBtn = screen.getByText('Save*')
      fireEvent.click(saveBtn)

      await waitFor(() => {
        expect(mockFs.writeTextFile).toHaveBeenCalled()
      })
    })
  })

  describe('Error Handling', () => {
    it('displays error when file read fails', async () => {
      const parentEntries = [{ name: 'bad.txt', isDirectory: false }]
      mockFs.readDir.mockResolvedValue(parentEntries as any)
      mockFs.readTextFile.mockRejectedValue(new Error('Permission denied'))

      render(<Editor />)

      const openBtn = screen.getByText('Open')
      fireEvent.click(openBtn)

      await waitFor(() => {
        expect(screen.getByText('bad.txt')).toBeInTheDocument()
      })

      const fileBtn = screen.getByText('bad.txt')
      fireEvent.click(fileBtn)

      await waitFor(() => {
        expect(screen.getByText(/Cannot read file/)).toBeInTheDocument()
      })
    })
  })
})