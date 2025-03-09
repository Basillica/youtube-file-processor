# Azure Event-Driven File Processing with Azure Functions, Blob Storage, and Terraform

This repository contains the code and Terraform configurations for building a scalable and efficient event-driven application for file storage and processing on Azure, as demonstrated in the YouTube video [Insert Your YouTube Video Link Here].

## Overview

This project showcases how to use Azure Functions, Azure Blob Storage, and Terraform to create a system that automatically processes files uploaded to a Blob Storage container.

**Key Components:**

* **Azure Blob Storage:** Stores the files to be processed.
* **Azure Event Grid:** Triggers an Azure Function when a new file is uploaded or updated in Blob Storage.
* **Azure Functions:** Processes the uploaded files.
* **Terraform:** Provisions and manages the Azure infrastructure.

## Architecture
+------------------+     +-------------------+     +---------------------+
|   Blob Storage   | --> |   Event Grid Topic  | --> |   Azure Function    |
+------------------+     +-------------------+     +---------------------+
| (File Uploads)   |     | (Storage Events)  |     | (File Processing)   |
+------------------+     +-------------------+     +---------------------+

## Prerequisites

* An Azure subscription.
* Azure CLI installed and configured.
* Terraform installed.
* An IDE or text editor (e.g., Visual Studio Code).

## Getting Started

1.  **Clone the Repository:**

    ```bash
    git clone git@github.com:Basillica/youtube-file-processor.git
    cd youtube-file-processor
    ```

2.  **Configure Terraform:**

    * Initialize Terraform:

        ```bash
        sh setup.dev.sh
        ```
        **You would need to first update the variables inside `setup.dev.sh` and `deploy.dev.sh`**

    * Update the variables inside `variables.dev.tfvars`

        **Note:** It is highly recommended to use Azure Managed Identities or Azure Key Vault for managing secrets in production environments.

3.  **Deploy the Azure Function:**
    * Deploy using the helper script
        ```bash
        sh deploy.dev.sh
        ```

    * Deploy the Azure Function using Azure CLI or Visual Studio Code:
        There is also a helper shell script for easy deployment using the CLI

4.  **Upload Files to Blob Storage:**

    * Upload files to the Blob Storage container you created.

5.  **Observe the Processing:**

    * Monitor the Azure Function logs to see the file processing in action.

## YouTube Video
https://www.youtube.com/watch?v=DtsBkT7JFFQ&lc=Ugz74ZmZoosWLSwf1Ql4AaABAg&ab_channel=EasyDevForAll