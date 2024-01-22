#!/bin/bash -u

# Copyright 2018 ConsenSys AG.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with
# the License. You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on
# an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the
# specific language governing permissions and limitations under the License.

NO_LOCK_REQUIRED=true

. ./.env
source "$(dirname "$0")/common.sh"

# Build and run containers and network
echo "docker-compose.yml" > ${LOCK_FILE}

echo "*************************************"
echo "Localnet"
echo "*************************************"
echo "Start network"
echo "--------------------"


echo "Starting network..."
docker compose --profile services build --pull

if [ "${1-}" = "--blockscout" -o "${1-}" = "-b" ]; then
  docker compose -f docker-compose.yml -f $BLOCKSCOUT_DOCKER_CONFIG --profile services up --detach
else
  docker compose --profile services up --detach
fi


#list services and endpoints
./$(dirname "$0")/list.sh
