import { Account, UpdateAccountRequest } from '@/dto/account';
import { createAccount, updateAccount as updateAccountApi, listAccount as listAccountApi, removeAccount as removeAccountApi } from '@/api/account'
import { defineStore } from "pinia";
// import { da } from 'element-plus/es/locale';
export const useAccountStore = defineStore('account', {
    state: () => ({ _list: [] as Account[], selectedProvider: 0 }),
    getters: {
        list: (state) => state._list,
        filterList: (state) => {
            if (!state.selectedProvider) {
                return state._list
            }
            return state._list.filter(
                (item) => item.provider === state.selectedProvider
            )
        }

    },
    actions: {
        setSelectProvider(provider: number) {
            this.selectedProvider = provider
        },
        async listAccount() {
            try {
                this._list = []
                const data = await listAccountApi();
                this._list = data
            } catch (err) {
                console.error(err)
                throw err
            }
        },
        async addAccount(account: Account) {
            try {
                //save to db
                await createAccount(account) as Account
                this._list.push(account)
            } catch (err) {
                console.error(err)
                throw err
            }
        },
        async updateAccount(account: Account) {
            try {
                //save to db
                await updateAccountApi(account)
                const index = this._list.findIndex((item) => item.id === account.id)
                this._list.splice(index, 1, account)
            } catch (err) {
                console.error(err)
                throw err
            }
        },
        async removeAccount(id: number) {
            try {
                console.log(id)
                //save to db
                await removeAccountApi(id)
                const index = this._list.findIndex((item) => item.id === id)
                this._list.splice(index, 1)
            } catch (err) {
                console.error(err)
                throw err
            }
        },
    }
})