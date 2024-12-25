# Home Data Collector

The "home data collector" is a personal IOT project to collect data from sensors which are registered in the local network.
## Description

This project is tailored to personal needs and sensor equipment used and does not suit a general application.
It is used to have an on premise database of all the IOT devices, rather than having to send their data to the cloud of the device.
The core component is the actix web backend service that provides REST API endpoints for operations like registering a new device, ingesting timeseries data or querying timeseries/meta data of the sensors.
To store all data (not only timeseries data), a surrealdb instance is used. Collection of the timeseries data is managed by a collector service that requests the sensor data at equal intervals and ingests them through the backend REST API's.

## Notes
This project is not finished yet and still under development.

## Deployment

In the current phase it is designed to run on a single host with docker-compose.
### Dependencies

* Docker - [Install Documentation](https://docs.docker.com/engine/install/)

### Running the service

* setup the database: 
```
sudo docker run --rm --pull always -p 8000:8000 \
    -v ./data/surrealdb:/data/surrealdb surrealdb/surrealdb:latest \
    start --user root --pass root rocksdb:data/surrealdb
```

* use the [Surrealist](https://surrealist.app/start) DB client to login as root:
    * create user to be used for the backend service - user has to be a "Database User" with edit permission
    * change/remove root level login params
* get the relevant files onto your device:
```
wget https://githubusercontent.com/reberfla/home-data-collector/master/compose.yml
wget https://githubusercontent.com/reberfla/home-data-collector/master/collector_config.yml
wget https://githubusercontent.com/reberfla/home-data-collector/master/backend_config.yml
```
* create `.env` file, add variables `DB_USER` AND `DB_PASS`
* ajdust the `collector_config.yml` and `backend_config.yml` to your needs
* create the needed directories on the host machine (if config not changed as below):
```
mkdir -p ./data/surrealdb
mkdir -p ./data/buffer
```
* run the application: `docker compose up`

## Author

* [@reberfla](https://github.com/reberfla)

## Version History
* no "release" yet - see [Notes](#notes)

## License

This project is licensed under the [MIT] License - see the [`LICENSE.md`](https://github.com/reberfla/home-data-collector/blob/master/LICENCE.md) for details.

## Acknowledgments

* [awesome-readme](https://github.com/matiassingers/awesome-readme)
* [example backend service with actix-web - by Code to the moon](https://www.youtube.com/watch?v=L8tWKqSMKUI&list=PLqnVCl9hPjM4wvPyuRerufBmaOTx7OMLo&index=5&t=938s)
* [Surrealist DB Client](https://surrealist.app/start)
