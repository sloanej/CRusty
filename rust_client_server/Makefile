default: server client

clean: 
	rm client server

server: server.rs
	rustc -C opt-level=3 server.rs -o server

client: client.rs
	rustc -C opt-level=3 client.rs -o client

