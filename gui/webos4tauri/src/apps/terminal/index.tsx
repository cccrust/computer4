import { useEffect, useRef, useState } from 'react'
import { Terminal as TerminalIcon } from 'lucide-react'
import { invoke } from '@tauri-apps/api/core'

export default function Terminal() {
  const containerRef = useRef<HTMLDivElement>(null)
  const terminalRef = useRef<any>(null)

  useEffect(() => {
    let terminal: any

    const init = async () => {
      const { Terminal: XTerminal } = await import('@xterm/xterm')
      const { FitAddon } = await import('@xterm/addon-fit')
      await import('@xterm/xterm/css/xterm.css')

      terminal = new XTerminal({
        cursorBlink: true,
        fontSize: 13,
        fontFamily: 'SF Mono, SFMono-Regular, ui-monospace, monospace',
        theme: {
          background: '#0d1117',
          foreground: '#c9d1d9',
          cursor: '#c9d1d9',
          black: '#0d1117',
          red: '#f85149',
          green: '#3fb950',
          yellow: '#d29922',
          blue: '#58a6ff',
          magenta: '#bc8cff',
          cyan: '#39c5cf',
          white: '#c9d1d9',
          brightBlack: '#484f58',
          brightRed: '#f85149',
          brightGreen: '#3fb950',
          brightYellow: '#d29922',
          brightBlue: '#58a6ff',
          brightMagenta: '#bc8cff',
          brightCyan: '#39c5cf',
          brightWhite: '#ffffff',
        },
        scrollback: 500,
      })

      const fitAddon = new FitAddon()
      terminal.loadAddon(fitAddon)

      if (containerRef.current) {
        terminal.open(containerRef.current)
        setTimeout(() => fitAddon.fit(), 50)
      }

      terminalRef.current = terminal

      terminal.writeln('\x1b[36mwebOS Terminal v0.1\x1b[0m')
      terminal.writeln('Type commands and press Enter to execute')
      terminal.writeln('Use arrow keys for command history')
      terminal.writeln('')
      terminal.write('$ ')

      let currentLine = ''
      let history: string[] = []
      let historyIndex = -1

      terminal.onData((data: string) => {
        const code = data.charCodeAt(0)

        if (data === '\r') {
          terminal.write('\r\n')
          if (currentLine.trim()) {
            history.push(currentLine)
            historyIndex = history.length
            executeCommand(currentLine.trim())
          } else {
            terminal.write('$ ')
          }
          currentLine = ''
        } else if (data === '\x7f') {
          if (currentLine.length > 0) {
            currentLine = currentLine.slice(0, -1)
            terminal.write('\b \b')
          }
        } else if (data === '\x1b[A') {
          if (historyIndex > 0) {
            historyIndex--
            clearLine(terminal, currentLine)
            currentLine = history[historyIndex]
            terminal.write(currentLine)
          }
        } else if (data === '\x1b[B') {
          if (historyIndex < history.length - 1) {
            historyIndex++
            clearLine(terminal, currentLine)
            currentLine = history[historyIndex]
            terminal.write(currentLine)
          }
        } else if (code >= 32) {
          currentLine += data
          terminal.write(data)
        }
      })

      const clearLine = (term: any, line: string) => {
        for (let i = 0; i < line.length; i++) term.write('\b \b')
      }

      const executeCommand = async (cmd: string) => {
        if (cmd === 'clear' || cmd === 'cls') {
          terminal.clear()
          terminal.write('$ ')
          return
        }
        if (cmd === 'exit' || cmd === 'quit') {
          terminal.writeln('\x1b[33mUse the Back button to close Terminal\x1b[0m')
          terminal.write('$ ')
          return
        }

        try {
          const output: string = await invoke('execute_shell', { command: cmd })
          const lines = output.replace(/\r\n/g, '\n').replace(/\r/g, '\n').split('\n')
          for (const line of lines) {
            terminal.writeln(line)
          }
        } catch (err) {
          terminal.writeln('\x1b[31m' + String(err) + '\x1b[0m')
        }
        terminal.write('$ ')
      }
    }

    init()

    return () => {
      if (terminalRef.current) {
        terminalRef.current.dispose()
      }
    }
  }, [])

  return (
    <div className="flex flex-col h-full bg-[#0d1117]">
      <div className="flex items-center gap-2 px-3 py-1.5 bg-[#161b22] border-b border-[#30363d]">
        <TerminalIcon size={12} className="text-gray-400" />
        <span className="text-[10px] font-mono text-gray-500">bash</span>
      </div>
      <div ref={containerRef} className="flex-1 p-2 overflow-hidden [&_.xterm]:h-full [&_.xterm]:outline-none" />
    </div>
  )
}