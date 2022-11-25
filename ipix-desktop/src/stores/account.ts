import { Account } from './../model/account';
import { createAccount, listAccount } from './../api/account'
import { defineStore } from "pinia";
import { da } from 'element-plus/es/locale';
export const useAccountStore = defineStore('account', {
    state: () => ({ _list: [] as Account[] }),
    getters: {
        list: (state) => state._list
    },
    actions: {
        async listAccount() {
            try {
                this._list = []
                const data = await listAccount();
                this._list = data
            } catch (err) {
                console.error(err)
            }
        },
        async addAccount(account: Account) {
            try {
                //save to db
               await createAccount(account)
                this._list.push(account)
                // console.log(data)
            } catch (err) {
                console.error(err)
                throw err
            }
        }
    }
})