import { Account } from '../model/account';
import { invoke } from '@tauri-apps/api/tauri';
export async function createAccount(account: Account) {
    return invoke('create_storate_account', { data: account });
}
export async function listAccount() :Promise<Account[]> {
    return invoke('list_all_storage_account')
}