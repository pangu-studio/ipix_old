import {Repository as MediaRepository} from '~/dto/repostiory'
import {invoke} from '@tauri-apps/api/tauri'

export async function listMediaRepository(): Promise<MediaRepository[]> { 
    return invoke('list_media_repository')
}
export async function addMediaRepository(repo: MediaRepository) {
    return invoke('add_media_repository', { data: repo })
}