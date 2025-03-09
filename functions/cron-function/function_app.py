import azure.functions as func
import logging
import os
import re
from azure.storage.blob import BlobServiceClient
from dotenv import load_dotenv


load_dotenv()
app = func.FunctionApp()


@app.function_name(name="health")
@app.route(route="health", methods=["GET", "OPTIONS"], auth_level="Anonymous")
def health(req: func.HttpRequest) -> func.HttpResponse:
    logging.info("hello from our awesome azure function")
    return func.HttpResponse(
            "healthy",
            status_code=200
    )

@app.function_name(name="eventGridTrigger")
@app.event_grid_trigger(arg_name="event")
def events(event: func.EventGridEvent):
    logging.info('Processing python EventGrid event: %s', event.get_json())
    payload = event.get_json()
    if payload["api"] == "PutBlod":
        url = payload["url"]
        match = re.search(r"(?<=https://)([^/]+)/(.*)", url)
        if match:
            blb_name = match.group(2)  # Capture only characters before .txt
            res = blb_name.split("/")
            container_name = res[0]
            blob_name = res[1]

            conn_str = os.environ.get("StorageAccountConnectionStringcd ", None)
            blob_service_client = BlobServiceClient.from_connection_string(conn_str=conn_str)

            local_path = "./data"
            # os.mkdir(local_path)
            local_file_name = "tonie.txt"

            download_file_path = os.path.join(local_path, str.replace(local_file_name ,'.txt', 'DOWNLOAD.txt'))
            container_client = blob_service_client.get_container_client(container=container_name)
            logging.info("\nDownloading blob to \n\t" + download_file_path)

            with open(file=download_file_path, mode="wb") as download_file:
                download_file.write(container_client.download_blob(blob_name).readall())

            logging.info("Blob name:", blb_name)

        else:
            logging.info("No match found")

    logging.info('Done processing python EventGrid event')


@app.timer_trigger(schedule="0 */1 * * * *", arg_name="timer", run_on_startup=False, use_monitor=False) 
def timer_trigger(timer: func.TimerRequest) -> None:
    if timer.past_due:
        logging.info('The timer is past due!')
    logging.info('Python timer trigger function executed.')