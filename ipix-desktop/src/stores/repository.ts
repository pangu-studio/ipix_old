import { Repository as MediaRepo } from "@/dto/repostiory";
import { listMediaRepository, addMediaRepository } from '@/api/repo';
import { defineStore } from "pinia";



export const useMediaRepositoryStore = defineStore('mediaRepository', {
    state: () => (
        {
            _list: [] as MediaRepo[],
        }),
    getters: {
        list: (state) => state._list,
    },
    actions: {
        async listMediaRepository() {
            try {
                this._list = []
                const data = await listMediaRepository();
                this._list = data
            } catch (err) {
                console.error(err)
                throw err
            }
        },
        async createMediaRepository(repo: MediaRepo) {
            try {
                await addMediaRepository(repo)
                this._list.unshift(repo)
            } catch (err) {
                console.error(err)
                throw err
            }
        }

    },
})