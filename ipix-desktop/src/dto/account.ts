export interface Account extends UpdateAccountRequest {

}
export interface CreateAccountRequest {
    name: string;
    app_key: string;
    secret: string;
    provider: number;
    addition: string
    deleted: boolean;
    description: string;
    create_time: string;
}
export interface UpdateAccountRequest extends CreateAccountRequest {
    id: number;
}