export enum KeyPolicy {
    Uuid = 0,
    DatetimeUuid,
    YyyyMmDdUuid,
  }
export enum RepoType {
    Picture = "Picture",
    Audio = "Audio",
    Video = "Video"
}
export interface Repository {
    id: string;
    name: string,
    description: string;
    repo_type: RepoType;
    addition: string;
    is_default: boolean;
    deleted: boolean;
    create_time: string;
    update_time: string;
}
