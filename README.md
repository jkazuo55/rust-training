# rust-training

register a tweet

curl -X POST -d "este es un tweet muy interesante" -H "Content-type: application/json" http://localhost:8000/tweets

curl -X POST -H "Content-type: application/json" http://localhost:8000/tweets/505328f9-6040-4a54-ab06-09f764172b00/likes