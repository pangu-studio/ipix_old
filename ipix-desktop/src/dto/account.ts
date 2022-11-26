export interface Account extends UpdateAccountRequest {

}
export interface CreateAccountRequest {
    name: string;
    app_key: string;
    secret: string;
    create_time: string;
    provider: number;
    addition: string
    deleted: boolean;
    description: string
}
export interface UpdateAccountRequest extends CreateAccountRequest {
    id: number;
}