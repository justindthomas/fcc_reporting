# FCC Reporting Service

This service gathers the information necessary to support the periodic reporting required by the FCC. It automates the retrieval of customer data from ChargeBee and provides an interface to upload the FCC Fabric data that is sent from the FCC's contractor to Emerald via email.

When the FCC data is submitted to the application, the Emerald customer data is retrieved and analyzed to build a set of relevant records. Those records are then used to create the CSV reports suitable to be uploaded to the FCC website.

## Running the Server

This is a Rust application and can be built and run from any system with a working Rust toolchain.

```
RUST_LOG=debug cargo run --bin server
```

Before executing the server, you will need to add some configuration data in the `.env` file at the root of this application.

```
API_URL=https://emeraldbroadband.chargebee.com/api/v2/subscriptions
API_KEY=live_TRUNCATED:
```

Note that the `API_KEY` above is truncated. Also note the trailling ':' character; that is required. This string is used as the Basic HTTP authentication parameter in the ChargeBee URL. That normally would take the format `username:password` but ChargeBee is just using the `username` field.

The `API_URL` should be as specified above.

## Standard Usage

At its most basic, this is a web page with a single upload element for the Fabric data with a display showing the completed reports. Once running, you can browse to http://localhost:8000 and you should be presented with a form to select and submit the FCC Fabric CSV file.

The form is currently completely unstyled. It's the minimum necessary to make the function work. I'll add some basic aesthetics to it when I have the core functionality working.

## API Usage

The functionality is accessible via API and can be accessed using curl in this way.

```
curl -F 'file=@./FCCFabric-FocusAreas.csv' http://localhost:8000/focus
```

Submitting the Fabric data as a multipart form to the `/focus` endpoint will kick off all of the analysis and report generation. This is exactly what submitting the file through the HTML form does as well.
