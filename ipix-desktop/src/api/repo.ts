import {Repository as MediaRepository} from '~/dto/repostiory'
import {invoke} from '@tauri-apps/api/tauri'

export async function listMediaRepository(): Promise<MediaRepository[]> { 
    return invoke('list_all_media_repository')
}
export async function findMediaRepository(id: string): Promise<MediaRepository> { 
    return invoke('find_media_repository',{id: id})
}
export async function addMediaRepository(repo: MediaRepository) {
    return invoke('create_media_repository', { data: repo })
}
export async function updateMediaRepository(repo: MediaRepository) {
    return invoke('update_media_repository', { data: repo })
}