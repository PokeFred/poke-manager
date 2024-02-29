import { invoke } from '@tauri-apps/api/tauri'

export type Ram = {
    memory: Memory
    swap: Swap
}

export type Memory = {
    total: number
    used: number
}

export type Swap = {
    total: number
    used: number
}

export async function getCurrentRamState(): Promise<Ram | string> {
    try {
        const response: Ram = await invoke<Ram>('get_ram')

        return response
    } catch (error) {
        return 'Error'
    }
}
