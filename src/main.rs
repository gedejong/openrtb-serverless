use lambda_http::{
    handler,
    lambda_runtime::{self, Context, Error},
    Request, Response,
};
use rusoto_core::Region;
use rusoto_kinesis::{KinesisClient, Kinesis, PutRecordInput};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_client = KinesisClient::new(Region::EuCentral1);
    let shared_client_ref = &shared_client;

    // Define a closure here that makes use of the shared client.
    let handler_func_closure = move |event: Request, _: Context| async move {
        let record = PutRecordInput {
            data: event.body().to_vec().into(),
            explicit_hash_key: None,
            partition_key: "default".to_string(),
            sequence_number_for_ordering: None,
            stream_name: "openrtb-kinesis-test".into()
        };
        shared_client_ref.put_record(record).await?;
        Ok(Response::builder()
                .status(204)
                .body(())
                .expect("failed to render response")
        )
    };

    lambda_runtime::run(handler(handler_func_closure)).await?;
    Ok(())
}
