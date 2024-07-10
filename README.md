
# Milkweed Mapper Server 🦋

## Overview

Milkweed Mapper Server is a Rust-based project designed to support an embedded system on a drone that captures continuous images of fields and categorizes the amount of milkweed. This project aims to monitor and promote the health of monarch butterflies.

## Features

- **Continuous Image Capture**: The system takes continuous pictures of fields.
- **Milkweed Detection**: Utilizes machine learning algorithms to detect and categorize milkweed.
- **Data Aggregation**: Collects and stores data for analysis.

### Installation

1. Clone the repository:
    ```
    git clone https://github.com/BradenEverson/milkweed_mapper_server.git
    cd milkweed_mapper_server
    ```

### Configuration

Modify the `check_locations.json` file to specify the locations to monitor. This can be done via submitting a `GET` request to your Raspberry Pi's IP at port 7878 and completing the GUI form or by sending a `POST` request to /collect-locs with a json array of circle objects with the following structure:

```
{
  "latitude": {f64},
  "longitude": {f64},
  "radius": {f64}
}
```

## Usage

Deploy the compiled binary to your Raspberry Pi mounted on the drone. Ensure the camera is properly connected and the drone is operational.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
