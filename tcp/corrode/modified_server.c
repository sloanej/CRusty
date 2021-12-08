/**
 * @file
 * @author [Nairit11](https://github.com/Nairit11)
 * @author [Krishna Vedala](https://github.com/kvedala)
 * @brief Server side implementation of Server-Client system.
 * @see client_server/client.c
 * 
 * Updated by Jack Sloan, Matthew Jackson, and Ishaan Gulati
 * 
 * 
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <netdb.h>
#include <unistd.h>
#include <sys/time.h>
#include <signal.h>
#include <time.h>

#define MAX 100000             /**< max. characters per message */
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
    // infinite loop for chat
    for (;;)
    {
        bzero(buff, MAX);

        rcount++;
        read(sockfd, buff, sizeof(buff));

        scount++;
        write(sockfd, payload, MAX);
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
/** Driver code */
int main()
{
    signal(SIGINT, handler);

    int sockfd, connfd;
    unsigned int len;
    struct sockaddr_in servaddr, cli;

    // socket create and verification
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd == -1)
    {
        perror("socket creation failed...\n");
        exit(0);
    }
    else
    {
        printf("Socket successfully created..\n");
    }
    bzero(&servaddr, sizeof(servaddr));

    // assign IP, PORT
    servaddr.sin_family = AF_INET;
    servaddr.sin_addr.s_addr = htonl(INADDR_ANY);
    servaddr.sin_port = htons(PORT);

    // Binding newly created socket to given IP and verification
    if ((bind(sockfd, (SA *)&servaddr, sizeof(servaddr))) != 0)
    {
        perror("socket bind failed...\n");
        exit(0);
    }
    else
    {
        printf("Socket successfully binded..\n");
    }

    // Now server is ready to listen and verification
    if ((listen(sockfd, 5)) != 0)
    {
        perror("Listen failed...\n");
        exit(0);
    }
    else
    {
        printf("Server listening..\n");
    }
    len = sizeof(cli);

    // Accept the data packet from client and verification
    connfd = accept(sockfd, (SA *)&cli, &len);
    if (connfd < 0)
    {
        perror("server acccept failed...\n");
        exit(0);
    }
    else
    {
        printf("server acccept the client...\n");
    }
    // Function for chatting between client and server
    func(connfd);

    // After chatting close the socket
    close(sockfd);
    return 0;
}
