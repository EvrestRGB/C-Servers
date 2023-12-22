#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define PORT 80
#define BUFFER_SIZE 1024

void handle_client(int client_socket);

int main() {
    int server_socket, client_socket;
    struct sockaddr_in server_addr, client_addr;
    socklen_t client_addr_len = sizeof(client_addr);

    // Create socket
    server_socket = socket(AF_INET, SOCK_STREAM, 0);
    if (server_socket == -1) {
        perror("Error creating socket");
        exit(EXIT_FAILURE);
    }

    // Set up server address struct
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);

    // Bind the socket to the specified port
    if (bind(server_socket, (struct sockaddr*)&server_addr, sizeof(server_addr)) == -1) {
        perror("Error binding socket");
        exit(EXIT_FAILURE);
    }

    // Listen for incoming connections
    if (listen(server_socket, 10) == -1) {
        perror("Error listening");
        exit(EXIT_FAILURE);
    }

    printf("Server listening on port %d...\n", PORT);

    // Accept and handle incoming connections
    while (1) {
        client_socket = accept(server_socket, (struct sockaddr*)&client_addr, &client_addr_len);
        if (client_socket == -1) {
            perror("Error accepting connection");
            continue;
        }

        handle_client(client_socket);

        close(client_socket);
    }

    close(server_socket);

    return 0;
}

void handle_client(int client_socket) {
    char buffer[BUFFER_SIZE];
    ssize_t bytes_received;

    // Receive the HTTP request
    bytes_received = recv(client_socket, buffer, sizeof(buffer), 0);
    if (bytes_received == -1) {
        perror("Error receiving data");
        return;
    }

    // Parse the request to get the requested file path
    // In a real-world scenario, you'd use a proper HTTP parser for this
    char *requested_file = "index.html"; // Default file
    if (strstr(buffer, "GET") != NULL) {
        char *start = strchr(buffer, '/') + 1;
        char *end = strchr(start, ' ');

        if (start != NULL && end != NULL && start < end) {
            size_t path_len = end - start;
            requested_file = strndup(start, path_len);
        }
    }

    // Open and read the requested file
    FILE *file = fopen(requested_file, "r");
    if (file == NULL) {
        perror("Error opening file");
        return;
    }

    // Prepare the HTTP response
    char response_header[200];
    snprintf(response_header, sizeof(response_header),
             "HTTP/1.1 200 OK\r\n"
             "Content-Type: text/html\r\n"
             "\r\n");

    // Send the HTTP response header
    send(client_socket, response_header, strlen(response_header), 0);

    // Send the contents of the requested file
    while ((bytes_received = fread(buffer, 1, sizeof(buffer), file)) > 0) {
        send(client_socket, buffer, bytes_received, 0);
    }

    fclose(file);
}