from kafka import KafkaProducer

import json
import time
from fake_user_data import get_registered_user

def json_serialize(data):
    return json.dumps(data).encode("utf-8")


producer = KafkaProducer(
    bootstrap_servers=["0.0.0.0:9092"],
    value_serializer=json_serialize
)

if __name__ == "__main__":
    while 1 == 1:
        registered_user = get_registered_user()
        print(registered_user)
        producer.send("register_landlords", registered_user)
        time.sleep(4)