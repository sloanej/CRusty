/**
 * @file
 * @author [Nairit11](https://github.com/Nairit11)
 * @author [Krishna Vedala](https://github.com/kvedala)
 * @brief Client side implementation of Server-Client system.
 * @see client_server/server.c
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <sys/socket.h>
#include <unistd.h>
#include <sys/time.h>
#include <signal.h>
#include <time.h>

#define MAX 100000       /**< max. characters per message */
#define PORT 8080          /**< port number to connect to */
#define SA struct sockaddr /**< shortname for sockaddr */

static int scount = 0;
static int rcount = 0;
static struct timeval start_time;

/**
 * Continuous loop to send and receive over the socket.
 * Exits when "exit" is sent from commandline.
 * @param sockfd socket handle number
 */
void func(int sockfd)
{
    char buff[MAX];
    //int n;
    char payload[MAX];
    srand(time(0));
    for(int i = 0; i < MAX; i++){
        payload[i] = rand() % 2 == 0 ? 'a' : 'b';
    }
    gettimeofday(&start_time, 0);
    for (;;)
    {
        scount++;
        write(sockfd, payload, MAX);
        bzero(buff, sizeof(buff));
        
        rcount++;
        read(sockfd, buff, sizeof(buff));
        
    }
}


static void handler(int _){
    (void)_;
    struct timeval tmptime;
    gettimeofday(&tmptime, 0);
    long total_time =    (tmptime.tv_usec / 1000 + tmptime.tv_sec * 1000) -    
                        (start_time.tv_usec / 1000 + start_time.tv_sec * 1000);

    float sspeed = ((float) scount) * MAX / total_time * (1000. / 1024);
    float rspeed = ((float)rcount) * MAX / total_time * (1000. / 1024);
    printf("\n%d payloads sent, %.2f kb/s\n%d payloads received, %.2f kb/s\nTime taken: %ld\n", scount, sspeed, rcount, rspeed, total_time);
    exit(EXIT_SUCCESS);
}


/**
 * @brief Driver code
 */
int main()
{
    signal(SIGINT, handler);


    int sockfd, connfd;
    struct sockaddr_in servaddr, cli;

    // socket create and verification
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd == -1)
    {
        printf("socket creation failed...\n");
        exit(0);
    }
    else
    {
        printf("Socket successfully created..\n");
    }
    bzero(&servaddr, sizeof(servaddr));

    // assign IP, PORT
    servaddr.sin_family = AF_INET;
    servaddr.sin_addr.s_addr = inet_addr("127.0.0.1");
    servaddr.sin_port = htons(PORT);

    // connect the client socket to server socket
    if (connect(sockfd, (SA *)&servaddr, sizeof(servaddr)) != 0)
    {
        printf("connection with the server failed...\n");
        exit(0);
    }
    else
    {
        printf("connected to the server..\n");
    }

    // function for chat
    func(sockfd);

    // close the socket
    close(sockfd);
    return 0;
}
