import { Repository as MediaRepo } from "@/dto/repostiory";
import { listMediaRepository, addMediaRepository ,updateMediaRepository, findMediaRepository} from '@/api/repo';
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
                let id = await addMediaRepository(repo) as string
                if (id) {
                     repo = await findMediaRepository(id)
                }else {
                    throw new Error("create repository failed")
                }
                this._list.unshift(repo)
            } catch (err) {
                console.error(err)
                throw err
            }
        },
        async updateMediaRepository(repo: MediaRepo) {
        
            try {
                await updateMediaRepository(repo);
                const index = this._list.findIndex((item) => item.id === repo.id);
                this._list.splice(index, 1, repo);
            } catch (err) {
                console.error(err);
                throw err;
            }
        }

    },
})