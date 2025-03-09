from azure.storage.blob import BlobServiceClient
import os
import re

# Replace with your information
url = "https://samplefunctoniestore.blob.core.windows.net/samplefunctonie-blob-container/aaa.txt"
match = re.search(r"(?<=https://)([^/]+)/(.*)", url)

if match:
    blb_name = match.group(2)  # Capture only characters before .txt
    res = blb_name.split("/")
    container_name = res[0]
    blob_name = res[1]
    print("Blob name:", blb_name)
    print(match, "------------")
else:
    print("No match found")

conn_str = "DefaultEndpointsProtocol=https;AccountName=samplefunctoniestore;AccountKey=V46ak5BN61hn3R8KZ9MYrdLUgLyGGOeaoLA9PTqVpxUmc1pmcTSYysG0EgkglVWNICKsvAeuBxss+AStc+0C9w==;EndpointSuffix=core.windows.net"
blob_service_client = BlobServiceClient.from_connection_string(conn_str=conn_str)

local_path = "./data"
# os.mkdir(local_path)
local_file_name = "tonie.txt"

download_file_path = os.path.join(local_path, str.replace(local_file_name ,'.txt', 'DOWNLOAD.txt'))
print(download_file_path)
container_client = blob_service_client.get_container_client(container=container_name)
print("\nDownloading blob to \n\t" + download_file_path)

with open(file=download_file_path, mode="wb") as download_file:
    print("w3hat the fuck --------------------")
    download_file.write(container_client.download_blob(blob_name).readall())