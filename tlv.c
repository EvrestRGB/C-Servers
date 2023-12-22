#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define MAX_BUFFER_SIZE 1024

void handle_client(int client_socket) {
    char buffer[MAX_BUFFER_SIZE];
    ssize_t received_bytes;

    // Receive TLV data
    received_bytes = recv(client_socket, buffer, sizeof(buffer), 0);
    if (received_bytes <= 0) {
        perror("Error receiving data");
        close(client_socket);
        return;
    }

    // Parse TLV data
    uint8_t tag = buffer[0];
    uint32_t length;
    memcpy(&length, buffer + 1, sizeof(uint32_t));
    length = ntohl(length);

    char value[length + 1];
    memcpy(value, buffer + 5, length);
    value[length] = '\0';

    // Process and print TLV data
    printf("Received TLV data:\n");
    printf("Tag: %u\n", tag);
    printf("Length: %u\n", length);
    printf("Value: %s\n", value);

    // Optionally, you can send a response back to the client
    // For example, echo the received TLV data
    send(client_socket, buffer, received_bytes, 0);

    // Close the client socket
    close(client_socket);
}

int main() {
    int server_socket, client_socket;
    struct sockaddr_in server_address, client_address;
    socklen_t client_address_len = sizeof(client_address);

    // Create socket
    server_socket = socket(AF_INET, SOCK_STREAM, 0);
    if (server_socket == -1) {
        perror("Error creating socket");
        exit(EXIT_FAILURE);
    }

    // Set up server address struct
    memset(&server_address, 0, sizeof(server_address));
    server_address.sin_family = AF_INET;
    server_address.sin_addr.s_addr = INADDR_ANY;
    server_address.sin_port = htons(8080); // You can choose any port number

    // Bind the socket
    if (bind(server_socket, (struct sockaddr*)&server_address, sizeof(server_address)) == -1) {
        perror("Error binding socket");
        close(server_socket);
        exit(EXIT_FAILURE);
    }

    // Listen for incoming connections
    if (listen(server_socket, 10) == -1) {
        perror("Error listening for connections");
        close(server_socket);
        exit(EXIT_FAILURE);
    }

    printf("Server listening on port 8080...\n");

    while (1) {
        // Accept a connection
        client_socket = accept(server_socket, (struct sockaddr*)&client_address, &client_address_len);
        if (client_socket == -1) {
            perror("Error accepting connection");
            continue;
        }

        // Handle the client in a separate function
        handle_client(client_socket);
    }

    // Close the server socket (this part is unreachable in this example)
    close(server_socket);

    return 0;
}
