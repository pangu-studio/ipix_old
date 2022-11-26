import { Account, CreateAccountRequest } from '@/dto/account';
import { invoke } from '@tauri-apps/api/tauri';
export async function createAccount(account: CreateAccountRequest) {
    return invoke('create_storate_account', { data: account });
}
export async function listAccount(): Promise<Account[]> {
    return invoke('list_all_storage_account')
}
export async function updateAccount(account: Account) {
    return invoke('update_storate_account', { data: account });
}
export async function removeAccount(id: number) {
    return invoke('remove_storage_account', { id: id });
}