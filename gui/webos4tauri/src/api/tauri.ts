import { invoke } from '@tauri-apps/api/core'
import type { AppInfo } from '../types/app'

export async function listApps(): Promise<AppInfo[]> {
  return invoke<AppInfo[]>('list_installed_apps')
}

export async function getAppManifest(appId: string): Promise<AppInfo> {
  return invoke<AppInfo>('get_app_manifest', { appId })
}

export async function getAppEntryUrl(appId: string): Promise<string> {
  return invoke<string>('get_app_entry_url', { appId })
}

export function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}
