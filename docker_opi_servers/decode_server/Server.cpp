#include <iostream>     
#include <stdlib.h>  
#include <cstdlib>         
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include "opencv2/opencv.hpp"
#include "opencv2/objdetect/objdetect.hpp"

#define BUF_LEN 65540 // Larger than maximum UDP packet size

using namespace cv;
using namespace std;
#include "config.h"

int main(int argc, char* argv[]) {
    cout << "Server running...: "<< endl;
    if (argc != 3) { // Test for correct number of parameters
        cerr << "Usage: " << argv[0] << " <UDP Port> <TCP & HTTP server ip>" << endl;
        exit(1);
    }
    const char* servAddress = argv[2]; // First arg: server address
    const unsigned short servPort = atoi(argv[1]); // First arg: local port UDP server
    
    while(1){
        // Create a TCP socket           
        sockaddr_in serverAddr;
        serverAddr.sin_family = AF_INET;
        serverAddr.sin_port = htons(12001);
        serverAddr.sin_addr.s_addr = inet_addr(servAddress);

        int sockfd = socket(AF_INET, SOCK_STREAM, 0);
        if (sockfd < 0) {
            std:cerr << "Error creating socket TCP" << endl;
            return 1;
        }
        
        if (connect(sockfd, (struct sockaddr*)&serverAddr, sizeof(serverAddr)) == -1) {
            cerr << "Error connecting to server tcp" << endl;
            close(sockfd);

            sleep(5);
            continue;
        }


        //Create a UDP socket
        sockaddr_in localAddr;
        memset(&localAddr, 0, sizeof(localAddr));
        localAddr.sin_family = AF_INET;
        localAddr.sin_addr.s_addr = htonl(INADDR_ANY);
        localAddr.sin_port = htons(servPort);

        int sock = socket(PF_INET,SOCK_DGRAM, IPPROTO_UDP);
        if (sock < 0){
            cerr << "Error creating Socket UDP" << endl;
            return 1;
        }
        //Bind to UDP socket
        if (bind(sock, (struct sockaddr *) &localAddr, sizeof(localAddr)) < 0) {
            cerr << "Error connecting to server UDP" << endl;
            close(sockfd);
            // Wait before attempting reconnection
            sleep(5);
            continue; // Retry the connection
        }

        char buffer[BUF_LEN]; // Buffer for echo string
        int recvMsgSize;      // Size of received message
        string sourceAddress; // Address of datagram source
        unsigned short sourcePort; // Port of datagram source

        cv::QRCodeDetector qrCodeDetector; // Create an instance of QR code detector
        chrono::high_resolution_clock::time_point lastQRProcessedTime = chrono::high_resolution_clock::now();

        while (1) {
            try {
                    // Block until receive message from a client
                do {
                    sockaddr_in clntAddr;
                    socklen_t addrLen = sizeof(clntAddr);
                    recvMsgSize = recvfrom(sock, (void *) buffer, BUF_LEN, 0, (sockaddr *) &clntAddr, (socklen_t *) &addrLen);
                    //recvMsgSize = sock.recvFrom(buffer, BUF_LEN, sourceAddress, sourcePort);
                } while (recvMsgSize > sizeof(int));
                int total_pack = ((int*)buffer)[0];

                //cout << "expecting length of packs: " << total_pack << endl;
                char* longbuf = new char[PACK_SIZE * total_pack];
                for (int i = 0; i < total_pack; i++) {
                    //recvMsgSize = sock.recvFrom(buffer, BUF_LEN, sourceAddress, sourcePort);
                    sockaddr_in clntAddr;
                    socklen_t addrLen = sizeof(clntAddr);
                    recvMsgSize = recvfrom(sock, (char *) buffer, BUF_LEN, 0, (sockaddr *) &clntAddr, (socklen_t *) &addrLen);
                    if (recvMsgSize != PACK_SIZE) {
                        cerr << "Received unexpected size pack: " << recvMsgSize << endl;
                        continue;
                    }
                    memcpy(&longbuf[i * PACK_SIZE], buffer, PACK_SIZE);
                }

                //cout << "Received packet from " << sourceAddress << ":" << sourcePort << endl;

                Mat rawData = Mat(1, PACK_SIZE * total_pack, CV_8UC1, longbuf);
                Mat frame = imdecode(rawData, IMREAD_COLOR);
                if (frame.size().width == 0) {
                    cerr << "decode failure!" << endl;
                    continue;
                }
                    // Calculate the time elapsed since the last QR code was processed
                chrono::high_resolution_clock::time_point currentTime = chrono::high_resolution_clock::now();
                chrono::duration<double> elapsedTime = currentTime - lastQRProcessedTime;

    
                // Process the received frame for QR code detection
                try{
                    string qrCodeData = qrCodeDetector.detectAndDecode(frame);
                    if (!qrCodeData.empty()) {
                        cout << "Detected QR Code: " << qrCodeData << endl;
                    
                        // Draw the detected QR code points
                        vector<Point> points;
                        bool found = qrCodeDetector.detect(frame, points);
                        if (found) {
                            for (int i = 0; i < points.size(); i++) {
                                circle(frame, points[i], 5, Scalar(0, 255, 0), -1);
                            }
                        }

                        // Draw the QR code data on the frame
                        Point qrDataPosition(10, 30);
                        putText(frame, qrCodeData, qrDataPosition, FONT_HERSHEY_SIMPLEX, 0.7, Scalar(255, 0, 0), 2);
                    
                        if (elapsedTime.count() >= 1.0) {

                        lastQRProcessedTime = currentTime; // Update the last processed time
                
                            // JSON data to send
                            string jsonData = R"({"qr": ")" + qrCodeData + R"("})";

                            // Construct the curl command
                            string command = "curl -d '" + jsonData + "' -H 'Content-Type: application/json' -X POST http://"+servAddress+":12000/qr";

                            // Execute the curl command using the system function
                            int result = system(command.c_str());

                            if (result == 0) {
                                cout << "POST request sent successfully" << endl;
    
                            } else {
                                cerr << "Failed to send POST request" << endl;
                            }
                        }

                    }
                }catch(cv::Exception& e){
                    cerr <<"QR detection error: " << e.what() << endl;
                    continue;
                }
                

                // Send the image data to the server
                // Encode the captured frame to JPEG format
                vector<uchar> encodedImageData;
                vector<int> compressionParams;
                compressionParams.push_back(IMWRITE_JPEG_QUALITY);
                compressionParams.push_back(90); // Adjust the JPEG quality as needed
                imencode(".jpg", frame, encodedImageData, compressionParams);
                send(sockfd, encodedImageData.data(), encodedImageData.size(), 0);


                delete[] longbuf;

            
            } catch (exception& e) {
                cerr << e.what() << endl;
                exit(1);
            }
        }
        // Close the socket
        close(sockfd);
    }
    return 0;
}
