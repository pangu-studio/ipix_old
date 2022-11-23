use anyhow::Result;
use qiniu_sdk::upload::{
    AlwaysSinglePart, AutoUploader, AutoUploaderObjectParams, MultiPartsUploaderWithCallbacks,
    UploadManager, UploadTokenSigner, UploadedPart, UploaderWithCallbacks, UploadingProgressInfo,
};
use qiniu_sdk::upload_token::{credential::Credential, UploadPolicy};
use std::time::Duration;
use uuid::Uuid;

//生成上传令牌
pub fn upload_token(key: &str) -> Option<String> {
    let access_key = "";
    let secret_key = "";
    let bucket_name = "";
    let credential = Credential::new(access_key, secret_key);
    let upload_token = UploadPolicy::new_for_object(bucket_name, key, Duration::from_secs(3600))
        .build_token(credential, Default::default());
    println!("{}", upload_token);

    return Some(upload_token.to_string());
}

//上传文件
pub fn upload_file(
    key: &str,
    access_key: &str,
    secret_key: &str,
    bucket_name: &str,
    prefix: &str,
    win: tauri::Window,
) -> Result<String> {
    let upload_progress = move |transfer: &UploadingProgressInfo| {
        if let Some(total_size) = transfer.total_bytes() {
            if (transfer.transferred_bytes() * 100 / total_size) % 5 == 0 {
                println!(
                    "Progress: {} / {} = {}%",
                    transfer.transferred_bytes(),
                    total_size,
                    transfer.transferred_bytes() * 100 / total_size
                );
                win.emit(
                    "upload_process",
                    transfer.transferred_bytes() * 100 / total_size,
                )
                .unwrap();
            }
        } else {
            println!("Progress: {}", transfer.transferred_bytes());
        }
        Ok(())
    };
    let part_uploaded = |part: &dyn UploadedPart| {
        println!(
            "Uploaded Part: {}, is resumed: {}",
            part.offset(),
            part.resumed()
        );
        Ok(())
    };

    println!(
        "Uploading file: {}, ak: {},sk: {}, bucket: {}",
        key, access_key, secret_key, bucket_name
    );
    let credential = Credential::new(access_key, secret_key);
    let upload_manager = UploadManager::builder(UploadTokenSigner::new_credential_provider(
        credential,
        bucket_name,
        Duration::from_secs(3600),
    ))
    .build();

    let id = Uuid::new_v4();
    let mut uploader: AutoUploader = upload_manager
        .auto_uploader_builder()
        .resumable_policy_provider(AlwaysSinglePart)
        .build();

    uploader
        .on_upload_progress(upload_progress)
        .on_part_uploaded(part_uploaded);

    let file_ex = &key[key.rfind(".").unwrap()..];
    let file_name = prefix.to_string() + id.to_string().as_str() + file_ex;
    let params = AutoUploaderObjectParams::builder()
        .object_name(file_name.to_owned())
        .file_name(file_name)
        .build();

    let up = uploader.upload_path(key, params).unwrap();

    println!("path: {}", up.to_string());
    Ok(up.to_string())
}
