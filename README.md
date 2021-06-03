TODO:
1. Create new server binary.
2. Extract out the timestamp info and stuff from the journal and place into the server.
3. The server should parse, if there is one, a config and extract info from that.
4. The server will do all the formatting prior to sending.
5. Get protobuf working.
6. Make async.
7. Pass in the hash map to read it can be mutable, no need to recreate.
    Consider updating the keys if they exists and removing those that do not exists in the current set difference between current key set and previous
