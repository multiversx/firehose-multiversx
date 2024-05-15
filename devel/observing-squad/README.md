# Setting up Multiversx infrastructure

In order to set up the infrastructure, the following two parts have to be in place:
- observing squad
- aggregator service


## Observing squad

An observing squad consists of a set of *Observer Nodes* (one for each shard,
including Metachain).

There is some [documentation](https://docs.multiversx.com/integrators/observing-squad/)
on the official docs on how to run an observing on the Multiversx network, but without
the proxy instace, we do not need a proxy instance for the current setup.

There are
also some notes in [README](https://github.com/multiversx/mx-chain-observing-squad/blob/master/README.md)
on [mx-chain-observing-squad](https://github.com/multiversx/mx-chain-observing-squad/blob/master/README.md) repository.

> Please note that we do not need a proxy instance (as stated in the above documentation)
> for our purposes.

### Setup with one machine

There is `external.toml` file is used to setup the observer -> aggregator connector.
The default values should be fine for the local testing/setup.
The `HostDriverConfig` section from `external.toml` config file can be updated if needed,
especially `URL` field.

There is a setup script that can be used to manage the entire setup with docker images,
on a single instance. It can be triggered based on the `run.sh` script.

In order to setup the folder structure run:
```bash
./run.sh setup
```

Run command will start the nodes:
```bash
./run.sh run
```

The nodes can be managed with `start` and `stop` commands:
```bash
./run.sh start

./run.sh stop
```

There is also a cleanup command, that will stop and remove the docker containers:
```bash
./run.sh cleanup
```

### Setup with multiple machines

#### Pull docker images

Pull images from [docker hub](https://hub.docker.com/u/multiversx).

```bash
docker pull multiversx/chain-observer:v1.7.10.0
docker pull multiversx/chain-keygenerator:latest
```

#### Generate PEM files

First, generate 4 PEM files, one for each Observer by running the keygenerator 4 times:

```bash
export KEYS_FOLDER=~/keys
docker run --rm --mount type=bind,source=${KEYS_FOLDER},destination=/keys --workdir /keys multiversx/chain-keygenerator:latest
```

After running the command 4 times, rename the resulted files to:

- `observerKey_0.pem`
- `observerKey_1.pem`
- `observerKey_2.pem`
- `observerKey_metachain.pem`

**Note:** the files will be owned by the `root` user. In order to `chown` them, do as follows:

```bash
sudo chown $(whoami) *
```

Copy external config and the generated pem file on each corresponding machine,
into `OBSERVER_DIR/config` folder (see the following section with an example
for `OBSERVER_DIR` variable)

#### Start Observers

On each machine:

* Start Observer of Shard 0:

```bash
export SHARD=0
export DISPLAY_NAME="MyObservingSquad-0"
export OBSERVER_DIR=~/MyObservingSquad/node-0
./run-observer.sh
```

* Start Observer of Shard 1:

```bash
export SHARD=1
export DISPLAY_NAME="MyObservingSquad-1"
export OBSERVER_DIR=~/MyObservingSquad/node-1
./run-observer.sh
```

* Start Observer of Shard 2:

```bash
export SHARD=2
export DISPLAY_NAME="MyObservingSquad-2"
export OBSERVER_DIR=~/MyObservingSquad/node-2
./run-observer.sh
```

* Start Observer of Metachain:

```bash
export SHARD=metachain
export DISPLAY_NAME="MyObservingSquad-metachain"
export OBSERVER_DIR=~/MyObservingSquad/node-metachain
./run-observer.sh
```
